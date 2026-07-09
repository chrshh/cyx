//! Native DRM/KMS + libinput + libseat backend for zonule.
//!
//! This is the piece that makes zonule cjyx's *real* display server (the role
//! tinywl filled), rather than smallvil's nested winit window. It is
//! deliberately minimal:
//!
//!   * single GPU  — the primary DRM node reported by udev/libseat
//!   * single output — the first connected connector, at its preferred mode
//!   * a plain `GlesRenderer` (no multi-GPU `GpuManager`/`MultiRenderer`)
//!   * a `GbmBufferedSurface` for scanout + an `OutputDamageTracker`
//!   * no dmabuf-feedback, no DRM leasing, no syncobj, no HW cursor plane
//!
//! Rendering is driven off a simple ~60 Hz calloop timer, gated on the vblank
//! of the previously queued frame. That is a touch busier than an ideal
//! repaint-scheduler, but it is small and predictable — a base to build on.

use std::time::{Duration, Instant};

use smithay::{
    backend::{
        allocator::{
            Fourcc,
            gbm::{GbmAllocator, GbmBufferFlags, GbmDevice},
        },
        drm::{DrmDevice, DrmDeviceFd, DrmEvent, GbmBufferedSurface},
        egl::{EGLContext, EGLDisplay},
        libinput::{LibinputInputBackend, LibinputSessionInterface},
        renderer::{
            Bind, damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer,
        },
        session::{Event as SessionEvent, Session, libseat::LibSeatSession},
        udev::{all_gpus, primary_gpu},
    },
    desktop::{Space, Window},
    output::{Mode as WlMode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::{EventLoop, LoopHandle},
        drm::control::{Device as ControlDevice, ModeTypeFlags, connector},
        input::Libinput,
        rustix::fs::OFlags,
        wayland_server::Display,
    },
    utils::{DeviceFd, Transform},
};

use crate::Zonule;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

/// Everything the backend owns for the one GPU + one output we drive. Stored on
/// [`Zonule`] so the calloop event-source callbacks (which get `&mut Zonule`)
/// can reach it alongside the compositor state.
pub struct Backend {
    pub session: LibSeatSession,
    drm: DrmDevice,
    renderer: GlesRenderer,
    gbm_surface: GbmBufferedSurface<GbmAllocator<DrmDeviceFd>, ()>,
    damage_tracker: OutputDamageTracker,
    output: Output,
    loop_handle: LoopHandle<'static, Zonule>,
    /// True between queueing a frame and its vblank; suppresses re-rendering so
    /// we don't stack buffers on the swapchain.
    waiting_for_vblank: bool,
    /// True while a render is already queued on the event loop's idle slot, so
    /// commit/vblank wake-ups don't schedule duplicate renders.
    render_queued: bool,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop: EventLoop<Zonule> = EventLoop::try_new()?;
    let display: Display<Zonule> = Display::new()?;
    let mut state = Zonule::new(&mut event_loop, display);

    // --- session: talk to seatd (via libseat) to become DRM master + open input.
    let (mut session, session_notifier) = LibSeatSession::new()?;
    let seat_name = session.seat();

    // --- pick the GPU: primary node, else the first udev reports.
    let gpu_path = primary_gpu(&seat_name)?
        .or_else(|| {
            all_gpus(&seat_name)
                .ok()
                .and_then(|mut v| v.drain(..).next())
        })
        .ok_or("no DRM device found")?;

    // --- open the DRM device through the session (so it survives VT switches).
    let fd = session.open(
        &gpu_path,
        OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK,
    )?;
    let drm_fd = DrmDeviceFd::new(DeviceFd::from(fd));
    let (mut drm, drm_notifier) = DrmDevice::new(drm_fd.clone(), true)?;
    let gbm = GbmDevice::new(drm_fd.clone())?;

    // --- GLES renderer on top of the GBM/EGL display.
    let egl_display = unsafe { EGLDisplay::new(gbm.clone())? };
    let egl_context = EGLContext::new(&egl_display)?;
    let renderer = unsafe { GlesRenderer::new(egl_context)? };

    // --- find the first connected connector, its preferred mode, and a CRTC.
    let res = drm.resource_handles()?;
    let connector = res
        .connectors()
        .iter()
        .filter_map(|handle| drm.get_connector(*handle, true).ok())
        .find(|conn| conn.state() == connector::State::Connected)
        .ok_or("no connected connector")?;
    let mode = *connector
        .modes()
        .iter()
        .find(|m| m.mode_type().contains(ModeTypeFlags::PREFERRED))
        .or_else(|| connector.modes().first())
        .ok_or("connector has no modes")?;
    let crtc = connector
        .encoders()
        .iter()
        .filter_map(|enc| drm.get_encoder(*enc).ok())
        .flat_map(|enc| res.filter_crtcs(enc.possible_crtcs()))
        .next()
        .ok_or("no CRTC available for connector")?;

    // --- scanout surface: DRM surface -> GBM double-buffering.
    let drm_surface = drm.create_surface(crtc, mode, &[connector.handle()])?;
    let allocator = GbmAllocator::new(
        gbm.clone(),
        GbmBufferFlags::RENDERING | GbmBufferFlags::SCANOUT,
    );
    let render_formats = renderer
        .egl_context()
        .dmabuf_render_formats()
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let gbm_surface = GbmBufferedSurface::new(
        drm_surface,
        allocator,
        &[Fourcc::Argb8888, Fourcc::Xrgb8888],
        render_formats,
    )?;

    // --- advertise the output to wayland clients and map it into the space.
    let (phys_w, phys_h) = connector.size().unwrap_or((0, 0));
    let output = Output::new(
        format!("{:?}-{}", connector.interface(), connector.interface_id()),
        PhysicalProperties {
            size: (phys_w as i32, phys_h as i32).into(),
            subpixel: Subpixel::Unknown,
            make: "cjyx".into(),
            model: "zonule".into(),
            serial_number: "0".into(),
        },
    );
    let _global = output.create_global::<Zonule>(&state.display_handle);
    let wl_mode = WlMode::from(mode);
    output.change_current_state(
        Some(wl_mode),
        Some(Transform::Normal),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(wl_mode);
    state.space.map_output(&output, (0, 0));

    let damage_tracker = OutputDamageTracker::from_output(&output);

    state.backend = Some(Backend {
        session,
        drm,
        renderer,
        gbm_surface,
        damage_tracker,
        output,
        loop_handle: event_loop.handle(),
        waiting_for_vblank: false,
        render_queued: false,
    });

    // --- input via libinput, opened through the same session.
    let mut libinput = Libinput::new_with_udev::<LibinputSessionInterface<LibSeatSession>>(
        state.backend.as_ref().unwrap().session.clone().into(),
    );
    libinput.udev_assign_seat(&seat_name).unwrap();
    let libinput_backend = LibinputInputBackend::new(libinput.clone());

    // --- wire up the event sources.
    event_loop
        .handle()
        .insert_source(libinput_backend, move |event, _, data: &mut Zonule| {
            data.process_input_event(event);
        })?;

    event_loop.handle().insert_source(
        drm_notifier,
        move |event, _, data: &mut Zonule| match event {
            DrmEvent::VBlank(_crtc) => {
                if let Some(backend) = data.backend.as_mut() {
                    let _ = backend.gbm_surface.frame_submitted();
                    backend.waiting_for_vblank = false;
                }
                // The just-scanned-out frame is done; draw the next one. This is
                // what phase-locks rendering to the display refresh (smooth),
                // instead of a free-running timer that beats against vblank.
                schedule_render(data);
            }
            DrmEvent::Error(err) => eprintln!("zonule: DRM error: {err:?}"),
        },
    )?;

    let mut session_libinput = libinput;
    event_loop
        .handle()
        .insert_source(
            session_notifier,
            move |event, _, data: &mut Zonule| match event {
                SessionEvent::PauseSession => {
                    session_libinput.suspend();
                    if let Some(backend) = data.backend.as_mut() {
                        backend.drm.pause();
                    }
                }
                SessionEvent::ActivateSession => {
                    let _ = session_libinput.resume();
                    if let Some(backend) = data.backend.as_mut() {
                        let _ = backend.drm.activate(false);
                        backend.gbm_surface.reset_buffers();
                        backend.waiting_for_vblank = false;
                    }
                }
            },
        )?;

    println!("zonule: running on {:?}", gpu_path);

    // Kick off the first frame (does the initial modeset). After that, rendering
    // is event-driven: each vblank schedules the next frame, and client commits
    // (see the compositor commit handler) wake a render when the display is idle.
    schedule_render(&mut state);

    // Launch the startup client (cjyx's cinit calls us as `display -s /bin/squishy`).
    spawn_startup(state.socket_name.clone());

    event_loop.run(None, &mut state, |state| {
        state.space.refresh();
        state.popups.cleanup();
        let _ = state.display_handle.flush_clients();
    })?;

    Ok(())
}

/// Parse `-s <cmd> [args...]` and spawn it as a wayland client, pointing it at
/// our compositor via `WAYLAND_DISPLAY`. This is how the shell/terminal comes up
/// under zonule (cinit passes `-s /bin/squishy`). If no `-s` is given, nothing
/// is launched — zonule just waits for clients on its socket.
fn spawn_startup(socket_name: std::ffi::OsString) {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-s" {
            if let Some(cmd) = args.next() {
                let rest: Vec<String> = args.collect();
                if let Err(err) = std::process::Command::new(&cmd)
                    .args(&rest)
                    .env("WAYLAND_DISPLAY", &socket_name)
                    .spawn()
                {
                    eprintln!("zonule: failed to spawn startup client {cmd:?}: {err}");
                }
            }
            return;
        }
    }
}

/// Request a repaint. Cheap and idempotent: it queues a single render on the
/// event loop's idle slot, so it can be called freely from client commits and
/// vblank events without piling up work. No-ops while a frame is already in
/// flight (waiting on vblank) or a render is already queued — those paths will
/// draw the latest state when they run.
///
/// This is the whole scheduler: renders happen exactly when there's something to
/// show (a commit) or a frame just finished (vblank), never on a free-running
/// clock. That's what makes it smooth and idle-quiet.
pub fn schedule_render(state: &mut Zonule) {
    let Some(backend) = state.backend.as_mut() else {
        return;
    };
    if backend.render_queued || backend.waiting_for_vblank {
        return;
    }
    backend.render_queued = true;
    backend.loop_handle.insert_idle(|state| {
        if let Some(backend) = state.backend.as_mut() {
            backend.render_queued = false;
        }
        if state.backend.is_some() {
            render_surface(
                &state.space,
                state.start_time,
                state.backend.as_mut().unwrap(),
            );
        }
    });
}

/// Render one frame of `space` onto the backend's output, queueing it for
/// scanout if anything changed. Takes disjoint borrows of `Zonule`'s fields so
/// the borrow checker is happy.
fn render_surface(space: &Space<Window>, start_time: Instant, backend: &mut Backend) {
    if backend.waiting_for_vblank || !backend.drm.is_active() {
        return;
    }

    let (mut dmabuf, age) = match backend.gbm_surface.next_buffer() {
        Ok(buf) => buf,
        Err(err) => {
            eprintln!("zonule: next_buffer failed: {err:?}");
            return;
        }
    };

    let mut framebuffer = match backend.renderer.bind(&mut dmabuf) {
        Ok(fb) => fb,
        Err(err) => {
            eprintln!("zonule: bind failed: {err:?}");
            return;
        }
    };

    let render_result = smithay::desktop::space::render_output::<
        _,
        WaylandSurfaceRenderElement<GlesRenderer>,
        _,
        _,
    >(
        &backend.output,
        &mut backend.renderer,
        &mut framebuffer,
        1.0,
        age as usize,
        [space],
        &[],
        &mut backend.damage_tracker,
        CLEAR_COLOR,
    );

    let result = match render_result {
        Ok(result) => result,
        Err(err) => {
            eprintln!("zonule: render failed: {err:?}");
            return;
        }
    };

    let damage = result.damage.cloned();
    let sync = result.sync;
    drop(framebuffer);

    if let Some(damage) = damage {
        if let Err(err) = backend
            .gbm_surface
            .queue_buffer(Some(sync), Some(damage), ())
        {
            eprintln!("zonule: queue_buffer failed: {err:?}");
            return;
        }
        backend.waiting_for_vblank = true;
    }

    // Let clients know they may draw their next frame.
    let elapsed = start_time.elapsed();
    for window in space.elements() {
        window.send_frame(&backend.output, elapsed, Some(Duration::ZERO), |_, _| {
            Some(backend.output.clone())
        });
    }
}

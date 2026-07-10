//! Nested "run it like a normal app" dev backend.
//!
//! Instead of taking over a real GPU as DRM master (see `tty.rs`), this opens a
//! window inside your existing Wayland/X11 session and runs zonule *inside* it.
//! Clients you launch connect to zonule's own wayland socket, so you can develop
//! window management — mapping, moving, resizing, launching apps — at native
//! speed with `cargo run --features dev`, no OS boot, no QEMU.
//!
//! This is essentially Smithay's `smallvil` winit backend, kept behind the `dev`
//! cargo feature so it never ships in the real image. Rendering here is a simple
//! continuous redraw loop (fine for a dev window); the native backend uses the
//! smarter vblank/commit-driven scheduler.

use std::time::Duration;

use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker,
        },
        winit::{self, WinitEvent},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{calloop::EventLoop, wayland_server::Display},
    utils::{Rectangle, Transform},
};

use crate::Zonule;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop: EventLoop<Zonule> = EventLoop::try_new()?;
    let display: Display<Zonule> = Display::new()?;
    let mut state = Zonule::new(&mut event_loop, display);

    init_winit(&mut event_loop, &mut state)?;

    println!(
        "zonule (winit/dev): nested compositor up, wayland socket = {:?}",
        state.socket_name
    );

    // Launch the startup client if invoked as `... -s <app>` (same flag the OS
    // uses). Launched clients get WAYLAND_DISPLAY pointed at *our* socket, so
    // they render into this window rather than the host session.
    crate::tty::spawn_startup(state.socket_name.clone());

    event_loop.run(None, &mut state, |_| {})?;
    Ok(())
}

fn init_winit(
    event_loop: &mut EventLoop<Zonule>,
    state: &mut Zonule,
) -> Result<(), Box<dyn std::error::Error>> {
    // Opens a window in the host session; picks Wayland or X11 automatically.
    let (mut backend, winit) = winit::init()?;

    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000,
    };

    let output = Output::new(
        "winit".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "cjyx".into(),
            model: "zonule-winit".into(),
            serial_number: "0".into(),
        },
    );
    let _global = output.create_global::<Zonule>(&state.display_handle);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    state.space.map_output(&output, (0, 0));

    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    event_loop
        .handle()
        .insert_source(winit, move |event, _, state| {
            match event {
                WinitEvent::Resized { size, .. } => {
                    output.change_current_state(
                        Some(Mode {
                            size,
                            refresh: 60_000,
                        }),
                        None,
                        None,
                        None,
                    );
                }
                WinitEvent::Input(event) => state.process_input_event(event),
                WinitEvent::Redraw => {
                    let size = backend.window_size();
                    let damage = Rectangle::from_size(size);

                    {
                        let focused = state.focused_window();
                        let (renderer, mut framebuffer) = backend.bind().unwrap();
                        let elements = crate::render::scene_elements(
                            renderer,
                            &state.space,
                            focused.as_ref(),
                            &state.cursor,
                            state.pointer.current_location(),
                            state.start_time.elapsed(),
                        );
                        smithay::desktop::space::render_output::<
                            _,
                            crate::render::ZonuleElement,
                            _,
                            _,
                        >(
                            &output,
                            renderer,
                            &mut framebuffer,
                            1.0,
                            0,
                            [&state.space],
                            &elements,
                            &mut damage_tracker,
                            [0.1, 0.1, 0.1, 1.0],
                        )
                        .unwrap();
                    }
                    backend.submit(Some(&[damage])).unwrap();

                    state.space.elements().for_each(|window| {
                        window.send_frame(
                            &output,
                            state.start_time.elapsed(),
                            Some(Duration::ZERO),
                            |_, _| Some(output.clone()),
                        )
                    });

                    state.space.refresh();
                    state.popups.cleanup();
                    let _ = state.display_handle.flush_clients();

                    // Ask for another frame — a simple continuous redraw loop.
                    backend.window().request_redraw();
                }
                WinitEvent::CloseRequested => {
                    state.loop_signal.stop();
                }
                _ => (),
            };
        })?;

    Ok(())
}

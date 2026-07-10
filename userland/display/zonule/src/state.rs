use std::{ffi::OsString, sync::Arc, time::Duration};

use smithay::{
    desktop::{PopupManager, Space, Window, WindowSurfaceType},
    input::{
        Seat, SeatState,
        pointer::{CursorImageStatus, PointerHandle},
    },
    reexports::{
        calloop::{EventLoop, Interest, LoopSignal, Mode, PostAction, generic::Generic},
        wayland_server::{
            Display, DisplayHandle,
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface,
        },
    },
    utils::{Logical, Point},
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        output::OutputManagerState,
        pointer_constraints::{PointerConstraintsHandler, with_pointer_constraint},
        seat::WaylandFocus,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
        socket::ListeningSocketSource,
    },
};

use crate::cursor::Cursor;

/// Data associated with a wayland client that connects to Zonule.
/// One instance of this type per client.
#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

// #[derive(Debug)]
pub struct Zonule {
    pub backend: Option<crate::tty::Backend>,
    pub start_time: std::time::Instant,
    pub socket_name: OsString,
    pub display_handle: DisplayHandle,

    // desktop
    pub space: Space<Window>,
    pub popups: PopupManager,

    // Smithay State
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Zonule>,
    pub data_device_state: DataDeviceState,
    pub loop_signal: LoopSignal,

    // input fields
    pub seat: Seat<Self>,
    pub cursor: Cursor,
    pub cursor_status: CursorImageStatus,
    pub pointer: PointerHandle<Zonule>,
    pub cursor_position_hint: Option<(WlSurface, Point<f64, Logical>)>,
}

impl PointerConstraintsHandler for Zonule {
    fn new_constraint(&mut self, surface: &WlSurface, pointer: &PointerHandle<Self>) {
        // Set restricted region for mouse
        let Some(curr_focus) = pointer.current_focus() else {
            return;
        };
        if curr_focus.wl_surface().as_deref() == Some(surface) {
            with_pointer_constraint(surface, pointer, |constraint| {
                constraint.unwrap().activate();
            });
        }
    }

    fn remove_constraint(&mut self, surface: &WlSurface, pointer: &PointerHandle<Self>) {
        if with_pointer_constraint(surface, pointer, |constraint| constraint.is_none()) {
            if let Some((hint_surface, hint_location)) = &self.cursor_position_hint {
                let origin = self
                    .space
                    .elements()
                    .find_map(|window| {
                        (window.wl_surface().as_deref() == Some(hint_surface))
                            .then(|| window.geometry())
                    })
                    .unwrap_or_default()
                    .loc
                    .to_f64();

                pointer.set_location(origin + *hint_location);
            }
            self.cursor_position_hint = None;
        }
    }

    fn cursor_position_hint(
        &mut self,
        surface: &WlSurface,
        pointer: &PointerHandle<Self>,
        location: Point<f64, Logical>,
    ) {
        if with_pointer_constraint(surface, pointer, |constraint| {
            constraint.is_some_and(|c| c.is_active())
        }) {
            self.cursor_position_hint = Some((surface.clone(), location));
        }
    }
}

impl Zonule {
    pub fn new(event_loop: &mut EventLoop<Self>, display: Display<Self>) -> Self {
        let dh = display.handle();
        let start_time = std::time::Instant::now();

        // Initialize protocols needed for displaying windows
        let compositor_state = CompositorState::new::<Self>(&dh);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);
        let shm_state = ShmState::new::<Self>(&dh, vec![]);
        let popups = PopupManager::default();

        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
        let data_device_state = DataDeviceState::new::<Self>(&dh);

        // init input
        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, "winit");

        let cursor = Cursor::load();
        let image = cursor.get_image(1, Duration::ZERO);
        let pointer = seat.add_pointer();

        seat.add_keyboard(Default::default(), 200, 25).unwrap();

        let space = Space::default();
        let socket_name = Self::init_wayland_listener(display, event_loop);
        let loop_signal = event_loop.get_signal();

        Self {
            backend: None,
            start_time,
            socket_name,
            display_handle: dh,

            space,
            popups,

            compositor_state,
            xdg_shell_state,
            shm_state,
            output_manager_state,
            seat_state,
            data_device_state,
            loop_signal,

            seat,
            cursor,
            cursor_status: CursorImageStatus::default_named(),
            pointer,
            cursor_position_hint: None,
        }
    }

    fn init_wayland_listener(
        display: Display<Zonule>,
        event_loop: &mut EventLoop<Self>,
    ) -> OsString {
        // Creates a new listening socket, automatically choosing the next available `wayland` socket name.
        let listening_socket = ListeningSocketSource::new_auto().unwrap();
        let socket_name = listening_socket.socket_name().to_os_string();
        let loop_handle = event_loop.handle();
        loop_handle
            .insert_source(listening_socket, move |client_stream, _, state| {
                state
                    .display_handle
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .unwrap();
            })
            .expect("Failed to init the wayland event source.");

        loop_handle
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |_, display, state| {
                    // Safety: we don't drop the display
                    unsafe {
                        display.get_mut().dispatch_clients(state).unwrap();
                    }
                    Ok(PostAction::Continue)
                },
            )
            .unwrap();

        socket_name
    }

    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<f64, Logical>)> {
        self.space
            .element_under(pos)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(s, p)| (s, (p + location).to_f64()))
            })
    }

    pub fn arrange(&mut self) {
        let Some(output) = self.space.outputs().next().cloned() else {
            return;
        };
        let geo = self.space.output_geometry(&output).unwrap();

        // Snapshot the windows: we can't hold the `elements()` borrow of `space`
        // while calling `map_element`, which needs `&mut space`.
        let windows: Vec<Window> = self.space.elements().cloned().collect();
        let n = windows.len() as i32;
        if n == 0 {
            return;
        }

        // Usable area = the output inset by GAP on all sides (the outer margin);
        // columns are then separated by GAP-wide gutters. The background shows
        // through the gutters, and window borders (see render::border_elements)
        // sit inside them.
        let gap = crate::render::GAP;
        let usable_x = geo.loc.x + gap;
        let usable_y = geo.loc.y + gap;
        let usable_w = geo.size.w - 2 * gap;
        let usable_h = geo.size.h - 2 * gap;
        // Each column gets an equal share of the width left after the (n-1)
        // inter-column gaps.
        let col_w = (usable_w - gap * (n - 1)) / n;

        for (i, window) in windows.iter().enumerate() {
            let i = i as i32;
            // The last column absorbs the rounding remainder so the row fills the
            // usable width exactly.
            let w = if i == n - 1 {
                usable_w - (col_w + gap) * (n - 1)
            } else {
                col_w
            };
            let loc = (usable_x + (col_w + gap) * i, usable_y);

            // Size = configure the client (it redraws at this size on its next
            // commit — a one-frame lag is normal). Position = place it now.
            if let Some(toplevel) = window.toplevel() {
                toplevel.with_pending_state(|state| {
                    state.size = Some((w, usable_h).into());
                });
                toplevel.send_pending_configure();
            }
            self.space.map_element(window.clone(), loc, false);
        }
    }

    pub fn focused_window(&self) -> Option<Window> {
        let surface = self.seat.get_keyboard()?.current_focus()?;
        self.space
            .elements()
            .find(|w| {
                w.toplevel()
                    .map(|t| t.wl_surface() == &surface)
                    .unwrap_or(false)
            })
            .cloned()
    }
}

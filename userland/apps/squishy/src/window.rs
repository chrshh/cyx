use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_keyboard, delegate_output, delegate_registry, delegate_seat,
    delegate_shm, delegate_xdg_shell, delegate_xdg_window,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{Capability, SeatHandler, SeatState, keyboard::KeyboardHandler},
    shell::{
        WaylandSurface,
        xdg::{
            XdgShell,
            window::{Window, WindowConfigure, WindowDecorations, WindowHandler},
        },
    },
    shm::{Shm, ShmHandler, slot::SlotPool},
};

use wayland_client::{
    Connection, QueueHandle,
    globals::registry_queue_init,
    protocol::{
        wl_keyboard::{self, WlKeyboard},
        wl_output, wl_seat, wl_shm, wl_surface,
    },
};

use crate::grid::Grid;
use crate::parser::{self, Cursor};
use crate::pty::{self, Pty};
use crate::render;
use crate::{font::FontCache, grid::Cell};
use calloop::{EventLoop, Interest, LoopHandle, Mode, PostAction, generic::Generic};
use calloop_wayland_source::WaylandSource;
use std::io::{Read, Write};
use std::os::fd::AsFd;

pub struct State {
    registry_state: RegistryState,
    output_state: OutputState,
    compositor_state: CompositorState,
    xdg_shell: XdgShell,
    shm: Shm,

    pool: SlotPool,
    window: Window,
    width: u32,
    height: u32,
    running: bool,
    first_configure: bool,

    font: FontCache,
    grid: Grid,

    pty: Pty,
    parser: vte::Parser,
    cursor: Cursor,
    needs_redraw: bool,

    seat_state: SeatState,
    keyboard: Option<wl_keyboard::WlKeyboard>,
}

pub fn run() {
    let conn = Connection::connect_to_env().expect("failed to connect to wayland compositor");
    let (globals, event_queue) =
        registry_queue_init::<State>(&conn).expect("failed to initialize registry");
    let qh = event_queue.handle();

    /* Bind globals */
    let compositor_state =
        CompositorState::bind(&globals, &qh).expect("wl_compositor not available");
    let xdg_shell = XdgShell::bind(&globals, &qh).expect("xdg_wm_base not available");
    let shm = Shm::bind(&globals, &qh).expect("wl_shm not available");
    let pool = SlotPool::new(256 * 256 * 4, &shm).expect("failed to create slot pool");

    let surface = compositor_state.create_surface(&qh);
    let window = xdg_shell.create_window(surface, WindowDecorations::RequestServer, &qh);
    window.set_title("squishy");
    window.set_app_id("squishy");
    window.set_min_size(Some((200, 100)));
    window.commit();

    let font = FontCache::new(16.0);
    let init_cols = 80;
    let init_rows = 24;
    let grid = Grid::new(init_cols, init_rows);
    let init_w = (init_cols * font.cell_w) as u32;
    let init_h = (init_rows * font.cell_h) as u32;

    let pty = pty::new(init_cols as u16, init_rows as u16).expect("pty");

    let mut state = State {
        registry_state: RegistryState::new(&globals),
        output_state: OutputState::new(&globals, &qh),
        compositor_state,
        xdg_shell,
        shm,
        pool,
        width: init_w,
        height: init_h,
        window,
        running: true,
        first_configure: true,
        font,
        grid,
        pty,
        parser: vte::Parser::new(),
        cursor: Cursor {
            col: 0,
            row: 0,
            cell: Cell::default(),
        },
        needs_redraw: false,
        seat_state: SeatState::new(&globals, &qh),
        keyboard: None,
    };

    let mut event_loop: EventLoop<State> = EventLoop::try_new().expect("calloop");
    let loop_handle = event_loop.handle();

    WaylandSource::new(conn.clone(), event_queue)
        .insert(loop_handle.clone())
        .expect("insert wayland source");

    let pty_fd = state
        .pty
        .file
        .as_fd()
        .try_clone_to_owned()
        .expect("clone fd");

    loop_handle
        .insert_source(
            Generic::new(pty_fd, Interest::READ, Mode::Level),
            |_event, _fd, state: &mut State| {
                let mut buf = [0u8; 4096];
                loop {
                    match (&state.pty.file).read(&mut buf) {
                        Ok(0) => {
                            state.running = false;
                            break;
                        }
                        Ok(n) => {
                            parser::feed(
                                &mut state.parser,
                                &mut state.grid,
                                &mut state.cursor,
                                &buf[..n],
                            );
                            state.needs_redraw = true;
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                        Err(_) => {
                            state.running = false;
                            break;
                        }
                    }
                }
                Ok(PostAction::Continue)
            },
        )
        .expect("insert pty source");

    while state.running {
        event_loop.dispatch(None, &mut state).expect("dispatch");

        if state.needs_redraw {
            state.draw(&qh);
            state.needs_redraw = false;
        }
    }
}

impl State {
    fn draw(&mut self, _qh: &QueueHandle<Self>) {
        let width = self.width as i32;
        let height = self.height as i32;
        let stride = width * 4;

        // Resize grid to match current window size in cells.
        let cols = (width as usize / self.font.cell_w).max(1);
        let rows = (height as usize / self.font.cell_h).max(1);
        if cols != self.grid.cols || rows != self.grid.rows {
            // Keep the demo strings on resize.
            self.grid.resize(cols, rows);
            self.grid.write_str(0, 0, "seaterm");
            self.grid.write_str(0, 1, "press anything (no input yet)");
            self.grid
                .write_str(0, 3, "the quick brown fox jumps over the lazy dog");
            self.grid
                .write_str(0, 4, "0123456789  !@#$%^&*()  []{}<>  =>  ->  ::");
        }

        let (buffer, canvas) = self
            .pool
            .create_buffer(width, height, stride, wl_shm::Format::Argb8888)
            .expect("failed to create buffer");

        render::paint(
            canvas,
            width as usize,
            height as usize,
            stride as usize,
            &self.grid,
            &mut self.font,
            &self.cursor,
        );

        let surface = self.window.wl_surface();
        surface.damage_buffer(0, 0, width, height);
        buffer.attach_to(surface).expect("failed to attach buffer");
        surface.commit();
    }
}
impl CompositorHandler for State {
    fn scale_factor_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: wl_output::Transform,
    ) {
    }

    fn frame(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: u32) {}

    fn surface_enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: &wl_output::WlOutput,
    ) {
    }
}

impl WindowHandler for State {
    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &Window) {
        self.running = false;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _window: &Window,
        configure: WindowConfigure,
        _serial: u32,
    ) {
        self.width = configure.new_size.0.map(|w| w.get()).unwrap_or(self.width);
        self.height = configure.new_size.1.map(|h| h.get()).unwrap_or(self.height);

        let cols = (self.width as usize / self.font.cell_w).max(1);
        let rows = (self.height as usize / self.font.cell_h).max(1);
        if cols != self.grid.cols || rows != self.grid.rows {
            self.grid.resize(cols, rows);

            let _ = rustix::termios::tcsetwinsize(
                &self.pty.file,
                rustix::termios::Winsize {
                    ws_row: rows as u16,
                    ws_col: cols as u16,
                    ws_xpixel: 0,
                    ws_ypixel: 0,
                },
            );
        }

        self.draw(qh);
        self.first_configure = false;
    }
}

impl ShmHandler for State {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl OutputHandler for State {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
}

impl ProvidesRegistryState for State {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}

impl SeatHandler for State {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let kb = self
                .seat_state
                .get_keyboard(qh, &seat, None)
                .expect("failed to create keyboard");
            self.keyboard = Some(kb);
        }
    }

    fn remove_capability(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard {
            if let Some(kb) = self.keyboard.take() {
                kb.release();
            }
        }
    }

    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
}

impl KeyboardHandler for State {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _serial: u32,
        _raw: &[u32],
        _keysyms: &[smithay_client_toolkit::seat::keyboard::Keysym],
    ) {
        println!("fn enter");
    }

    fn leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,

        _serial: u32,
    ) {
        println!("fn leave");
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,

        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,

        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        if let Some(text) = &event.utf8 {
            let _ = self.pty.file.write(text.as_bytes());
        }
        self.needs_redraw = true;
    }

    fn release_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,

        _event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        println!("fn release_key");
        self.needs_redraw = true;
    }

    fn update_modifiers(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _modifiers: smithay_client_toolkit::seat::keyboard::Modifiers,
        _raw_modifiers: smithay_client_toolkit::seat::keyboard::RawModifiers,
        _layout: u32,
    ) {
        println!("fn update_modifiers");
    }

    fn repeat_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        println!("fn repeat_key");
    }
}

delegate_compositor!(State);
delegate_xdg_shell!(State);
delegate_xdg_window!(State);
delegate_shm!(State);
delegate_registry!(State);
delegate_output!(State);
delegate_keyboard!(State);
delegate_seat!(State);

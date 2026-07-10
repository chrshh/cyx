#![allow(irrefutable_let_patterns)]

mod cursor;
mod grabs;
mod handlers;
mod input;
mod keyboard;
mod render;
mod state;
mod tty;
#[cfg(feature = "dev")]
mod winit;

pub use state::Zonule;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    // Dev builds (`cargo run --features dev`): if we're launched inside an
    // existing graphical session, run as a NESTED compositor in a window via the
    // winit backend — iterate on window management without booting the OS. This
    // branch doesn't exist in the shipped binary (dev feature off).
    #[cfg(feature = "dev")]
    if std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some() {
        return winit::run();
    }

    // Otherwise (and always in the shipped image): drive the display + input
    // directly via libseat + udev + libinput (native DRM/KMS). This is what
    // cinit launches as `/bin/display`.
    tty::run()
}

fn init_logging() {
    tracing_subscriber::fmt().init();
}

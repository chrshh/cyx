#![allow(irrefutable_let_patterns)]

mod grabs;
mod handlers;
mod input;
mod state;
mod tty;

pub use state::Zonule;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    // zonule is a native DRM/KMS compositor: it drives the display and input
    // directly via libseat + udev + libinput. It expects to be launched inside a
    // seatd session (cjyx's cinit runs it under seatd-launch).
    tty::run()
}

fn init_logging() {
    tracing_subscriber::fmt().init();
}

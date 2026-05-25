mod cpu;
mod mem;

use crossterm::event;

use crate::{
    cpu::{Cpu, init_cpu, update_cpu},
    mem::{Mem, init_mem, update_mem},
};
use std::time::{Duration, Instant};

pub struct App {
    cpu: Cpu,
    mem: Mem,
    running: bool,
}

pub fn update_stats(app: &mut App) {
    update_cpu(app);
    update_mem(app);
}

pub fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let cpu: Cpu = init_cpu();
    let mem: Mem = init_mem();

    let mut app = App {
        cpu,
        mem,
        running: true,
    };

    let tick_rate = Duration::from_secs(2);
    let mut last_tick = Instant::now();

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if !app.running {
            break Ok(());
        }

        if event::poll(timeout)? && event::read()?.is_key_press() {
            break Ok(());
        }

        if last_tick.elapsed() >= tick_rate {
            update_stats(&mut app);
            // println!("CPU: {}", &app.cpu.percent_free);
            println!("MEM: {}", &app.mem.mem_total);
            last_tick = Instant::now();
        }

        terminal
            .draw(|frame| frame.render_widget("Hello World!", frame.area()))
            .unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

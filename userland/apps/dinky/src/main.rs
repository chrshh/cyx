mod cpu;
mod mem;
mod uptime;

use crossterm::event;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Stylize},
    widgets::{Block, BorderType, Cell, Row, Table, Widget},
};

use crate::{
    cpu::{Cpu, CpuStats},
    mem::Mem,
    uptime::Uptime,
};
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug, Clone, Copy, Default)]
pub struct App {
    pub cpu: CpuStats,
    pub mem: Mem,
    pub uptime: Uptime,
    running: bool,
}

pub enum Update {
    Cpu(CpuStats),
    Mem(Mem),
    Uptime(Uptime),
}

fn spawn_workers() -> mpsc::Receiver<Update> {
    let (tx, rx) = mpsc::channel();

    /* cpu */
    let tx_cpu = tx.clone();
    thread::spawn(move || {
        let mut cpu = Cpu::new().unwrap();
        loop {
            let pct = cpu.tick().unwrap();
            if tx_cpu.send(Update::Cpu(pct)).is_err() {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* mem */
    let tx_mem = tx.clone();
    thread::spawn(move || {
        loop {
            match Mem::read() {
                Ok(mem) => {
                    if tx_mem.send(Update::Mem(mem)).is_err() {
                        break;
                    }
                }
                Err(_) => continue,
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* uptime */
    let tx_uptime = tx.clone();
    thread::spawn(move || {
        loop {
            match Uptime::read() {
                Ok(uptime) => {
                    if tx_uptime.send(Update::Uptime(uptime)).is_err() {
                        break;
                    }
                }
                Err(_) => continue,
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    drop(tx);
    rx
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

pub fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut app = App {
        running: true,
        ..Default::default()
    };

    let rx = spawn_workers();
    terminal.draw(|f| render(f, &app))?;
    loop {
        if !app.running {
            break Ok(());
        }
        while let Ok(update) = rx.try_recv() {
            match update {
                Update::Cpu(c) => app.cpu = c,
                Update::Mem(m) => app.mem = m,
                Update::Uptime(u) => app.uptime = u,
            }
        }
        if event::poll(Duration::from_millis(1000))? && event::read()?.is_key_press() {
            break Ok(());
        }
        terminal.draw(|f| render(f, &app))?;
    }
}

fn render(frame: &mut Frame, app: &App) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Green)
        .render(border_area, frame.buffer_mut());

    let mut rows = Vec::new();

    /* CPU header */
    rows.push(Row::new(vec![
        Cell::from("CPU")
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
        Cell::from(""),
    ]));

    /* CPU metrics */
    rows.push(Row::new(vec![
        Cell::from(" % Used"),
        Cell::from(format!("{:.2}%", app.cpu.percent_used)),
    ]));

    rows.push(Row::new(vec![
        Cell::from("────────────────────────").fg(Color::DarkGray),
        Cell::from("────────────────────────").fg(Color::DarkGray),
    ]));

    /* Memory header */
    rows.push(Row::new(vec![
        Cell::from("Memory")
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
        Cell::from(""),
    ]));

    let (mem_total, total_ab) = Mem::format_bytes(app.mem.mem_total);
    let (mem_aval, aval_ab) = Mem::format_bytes(app.mem.mem_avail);

    /* Memory metrics */
    rows.push(Row::new(vec![
        Cell::from(" Total"),
        Cell::from(format!("{:.2} {}", mem_total, total_ab)),
    ]));

    rows.push(Row::new(vec![
        Cell::from(" Free"),
        Cell::from(format!("{:.2} {}", mem_aval, aval_ab)),
    ]));

    /* Uptime header */
    rows.push(Row::new(vec![
        Cell::from("Uptime")
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
        Cell::from(""),
    ]));

    rows.push(Row::new(vec![
        Cell::from("────────────────────────").fg(Color::DarkGray),
        Cell::from("────────────────────────").fg(Color::DarkGray),
    ]));

    let upt: String = Uptime::format_uptime(app.uptime.total_uptime);

    /* Uptime metrics */
    rows.push(Row::new(vec![
        Cell::from(" Uptime"),
        Cell::from(upt.to_string()),
    ]));

    rows.push(Row::new(vec![
        Cell::from(" Current/Total"),
        Cell::from(format!(
            "{}/{}",
            app.uptime.current_execs, app.uptime.total_execs
        )),
    ]));

    let widths = [Constraint::Percentage(40), Constraint::Percentage(60)];
    let table = Table::new(rows, widths).block(Block::default());

    frame.render_widget(table, inner_area);
}

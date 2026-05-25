use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::App;

#[derive(Debug)]
pub struct Cpu {
    pub percent_free: f32,
    pub prev_total: f32,
    pub prev_idle: f32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            percent_free: 0.00,
            prev_total: 0.00,
            prev_idle: 0.00,
        }
    }
}

impl Cpu {
    fn read_cpu_proc(cpu: &mut Cpu, new: bool) -> io::Result<()> {
        let file = File::open("/proc/stat")?;
        let mut reader = BufReader::new(file);

        let mut proc = String::new();
        let _ = reader.read_line(&mut proc)?;
        let v: Vec<&str> = proc.split_whitespace().collect();

        let idle_str = v[4];
        let iowait_str = v[3];
        let user_str = v[1];
        let nice_str = v[2];
        let sys_str = v[3];
        let irq_str = v[6];
        let soft_irq_str = v[7];
        let steal_str = v[8];

        let idle: f32 = idle_str.parse().unwrap();
        let iowait: f32 = iowait_str.parse().unwrap();
        let user: f32 = user_str.parse().unwrap();
        let nice: f32 = nice_str.parse().unwrap();
        let sys: f32 = sys_str.parse().unwrap();
        let irq: f32 = irq_str.parse().unwrap();
        let soft_irq: f32 = soft_irq_str.parse().unwrap();
        let steal: f32 = steal_str.parse().unwrap();

        let idle_total = idle + iowait;
        let non_idle = user + nice + sys + irq + soft_irq + steal;

        let total: f32 = idle_total + non_idle;

        if new {
            cpu.prev_total = total;
            cpu.prev_idle = idle_total;
        } else {
            let total_delta = total - cpu.prev_total;
            let idle_delta = idle_total - cpu.prev_idle;

            cpu.percent_free = (total_delta - idle_delta) / (total_delta * 100.00);
            cpu.prev_total = total;
            cpu.prev_idle = idle_total;
        }

        Ok(())
    }
}

/* initial check to populate struct */
pub fn init_cpu() -> Cpu {
    let mut cpu = Cpu::default();
    Cpu::read_cpu_proc(&mut cpu, true).unwrap();
    cpu
}

pub fn update_cpu(app: &mut App) {
    Cpu::read_cpu_proc(&mut app.cpu, false).unwrap();
}

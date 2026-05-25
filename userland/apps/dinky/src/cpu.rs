use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Copy, Clone, Default)]
pub struct Cpu {
    pub prev_total: u64,
    pub prev_idle: u64,
}

impl Cpu {
    fn sample() -> io::Result<(u64, u64)> {
        let file = File::open("/proc/stat")?;
        let mut line = String::new();
        BufReader::new(file).read_line(&mut line)?;

        let v: Vec<&str> = line.split_whitespace().collect();

        let user: u64 = v[1].parse().unwrap();
        let nice: u64 = v[2].parse().unwrap();
        let system: u64 = v[3].parse().unwrap();
        let idle: u64 = v[4].parse().unwrap();
        let iowait: u64 = v[5].parse().unwrap();
        let irq: u64 = v[6].parse().unwrap();
        let softirq: u64 = v[7].parse().unwrap();
        let steal: u64 = v[8].parse().unwrap();

        let idle_total = idle + iowait;
        let non_idle = user + nice + system + irq + softirq + steal;
        let total = idle_total + non_idle;

        Ok((idle_total, total))
    }

    pub fn new() -> io::Result<Self> {
        let (idle, total) = Self::sample()?;
        Ok(Self {
            prev_idle: idle,
            prev_total: total,
        })
    }

    pub fn tick(&mut self) -> io::Result<f32> {
        let (idle, total) = Self::sample()?;
        let total_delta = total.saturating_sub(self.prev_total) as f32;
        let idle_delta = idle.saturating_sub(self.prev_idle) as f32;
        self.prev_total = total;
        self.prev_idle = idle;

        let percent_used = if total_delta > 0.0 {
            (total_delta - idle_delta) / total_delta * 100.0
        } else {
            0.0
        };

        Ok(percent_used)
    }
}

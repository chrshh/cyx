use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Default, Copy, Clone)]
pub struct Mem {
    pub mem_total: u64,
    pub mem_avail: u64,
}

impl Mem {
    pub fn read() -> io::Result<Self> {
        let file = File::open("/proc/meminfo")?;
        let reader = BufReader::new(file);

        let mut mem_total = 0;
        let mut mem_avail = 0;

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("MemTotal:") => mem_total = parts.next().unwrap().parse().unwrap(),
                Some("MemAvailable:") => mem_avail = parts.next().unwrap().parse().unwrap(),
                _ => continue,
            }
            if mem_total != 0 && mem_avail != 0 {
                break;
            }
        }

        Ok(Self {
            mem_total,
            mem_avail,
        })
    }

    pub fn format_bytes(kb: u64) -> (f32, &'static str) {
        let kb = kb as f32;
        if kb >= 1024.0 * 1024.0 {
            (kb / (1024.0 * 1024.0), "GB")
        } else if kb >= 1024.0 {
            (kb / 1024.0, "MB")
        } else {
            (kb, "KB")
        }
    }
}

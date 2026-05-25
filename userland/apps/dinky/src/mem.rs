use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::App;

#[derive(Debug)]
pub struct Mem {
    pub mem_total: f32,
    pub mem_avail: f32,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            mem_total: 0.00,
            mem_avail: 0.00,
        }
    }
}

impl Mem {
    fn read_from_meminfo(mem: &mut Mem) -> io::Result<()> {
        let file = File::open("/proc/meminfo")?;
        let mut reader = BufReader::new(file);

        let mut meminfo_0 = String::new();
        let mut meminfo_2 = String::new();
        let mut poop = String::new();

        let _ = reader.read_line(&mut meminfo_0)?;
        let _ = reader.read_line(&mut poop);
        let _ = reader.read_line(&mut meminfo_2)?;

        let v: Vec<&str> = meminfo_0.split_whitespace().collect();
        let x: Vec<&str> = meminfo_2.split_whitespace().collect();

        let info0: f32 = v[1].parse().unwrap();
        let info2: f32 = x[1].parse().unwrap();

        mem.mem_total = info0;
        mem.mem_avail = info2;

        Ok(())
    }
}

pub fn init_mem() -> Mem {
    let mut mem = Mem::default();
    Mem::read_from_meminfo(&mut mem).unwrap();
    mem
}

pub fn update_mem(app: &mut App) {
    Mem::read_from_meminfo(&mut app.mem).unwrap();
}

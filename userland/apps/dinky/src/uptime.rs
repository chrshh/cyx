use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Uptime {
    pub total_uptime: u64,
    pub total_execs: u64,
    pub current_execs: u64,
}

impl Uptime {
    pub fn read() -> io::Result<Self> {
        /* total uptime */
        let upt_file = File::open("/proc/uptime")?;
        let mut upt_line = String::new();
        BufReader::new(upt_file).read_line(&mut upt_line)?;

        let u: Vec<&str> = upt_line.split_whitespace().collect();
        let total_uptime = u[0].parse::<f64>().unwrap() as u64;

        /* current & total execs */
        let exec_file = File::open("/proc/loadavg")?;
        let mut exec_line = String::new();
        BufReader::new(exec_file).read_line(&mut exec_line)?;

        let v: Vec<&str> = exec_line.split_whitespace().collect();
        let exec_str: Vec<&str> = v[3].split('/').collect();
        let current_execs = exec_str[0].parse().unwrap();
        let total_execs = exec_str[1].parse().unwrap();

        Ok(Self {
            total_uptime,
            total_execs,
            current_execs,
        })
    }

    pub fn format_uptime(total_uptime: u64) -> String {
        let mut res = String::new();

        let hours: u64 = total_uptime / 3600;

        let rem = total_uptime % 3600;
        let minutes = rem / 60;
        let seconds = rem % 60;

        res.push_str(&hours.to_string());
        res.push(':');
        res.push_str(&minutes.to_string());
        res.push(':');
        res.push_str(&seconds.to_string());

        res
    }
}

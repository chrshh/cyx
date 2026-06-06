use crate::{err::GrError, input::Argv, writer::Output};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Default)]
pub struct Match {
    pub filename: String,
    pub complement: String,
    pub line_num: u32,
}

#[derive(Debug, Default)]
pub struct MatchSet {
    pub matches: Vec<Match>,
}

impl Match {
    pub fn construct_without_flags(c: &String) -> Self {
        Self {
            filename: String::new(),
            complement: c.clone(),
            line_num: 0,
        }
    }
}

impl MatchSet {
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    pub fn add(&mut self, m: Match) {
        self.matches.push(m);
    }
}

pub fn search(args: Argv) -> std::io::Result<u32> {
    let match_set = MatchSet::new();

    if args.flags == 0 {
        search_without_flags(args);
    } else {
        search_with_flags(args);
    }
    Ok(0)
}

pub fn search_without_flags(args: Argv) -> std::io::Result<()> {
    let path = Path::new(&args.path);

    let _: () = if path.is_dir() {
        GrError::call_exit(GrError::IsDir(path.to_string_lossy()))
    };

    let _: () = if !path.exists() {
        GrError::call_exit(GrError::FileNotFound(path.to_string_lossy()))
    };

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let curr_line = &line?;
        if curr_line.contains(&args.query) {
            let mut m = Match::construct_without_flags(curr_line);
            m.complement = curr_line.clone();
            let o = Output::default();
            o.build_output(&mut m, &args);
        }
    }

    Ok(())
}

pub fn search_with_flags(args: Argv) {}

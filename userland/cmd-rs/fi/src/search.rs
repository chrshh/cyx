use std::{
    fs::{File, metadata, read_dir, read_to_string},
    io::{self, BufWriter, Read, Write, stdout},
    path::Path,
};

use crate::{error::CGrepError, flags::RECURSIVE, parse::Config, writer::Output};

pub struct SearchResult {
    pub filepath: String,
    pub complement: String,
    pub line_num: usize,
}

impl SearchResult {
    pub fn new(filepath: String, complement: String, line_num: usize) -> Self {
        Self {
            filepath,
            complement,
            line_num,
        }
    }
}

/* main entry point for searching */
pub fn search(cfg: &Config) -> Result<bool, CGrepError> {
    if !Path::new(&cfg.path).exists() {
        return Err(CGrepError::NotFound(cfg.path.clone()));
    }
    let meta = metadata(&cfg.path)?;

    if meta.is_dir() && cfg.flags & RECURSIVE == 0 {
        return Err(CGrepError::IsDir(cfg.path.clone()));
    }

    let mut out = BufWriter::new(stdout().lock());

    let found = if meta.is_dir() {
        search_dir(&cfg.path, cfg, &mut out)?
    } else {
        search_file(&cfg.path, cfg, &mut out)?
    };

    Ok(found)
}

/* iterates through entries and calls search_file when a file is found */
pub fn search_dir<W: Write>(path: &str, cfg: &Config, out: &mut W) -> Result<bool, CGrepError> {
    let mut found = false;

    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.path().is_file() {
            found |= search_file(entry.path().to_str().unwrap(), cfg, out)?;
        } else {
            found |= search_dir(entry.path().to_str().unwrap(), cfg, out)?;
        }
    }
    Ok(found)
}

pub fn search_file<W: Write>(
    path: &str,
    cfg: &Config,
    mut out: &mut W,
) -> Result<bool, CGrepError> {
    /* skips binary files and returns false */
    let mut found = false;
    if is_binary_heuristic(path)? {
        return Ok(found);
    }

    let mut w = Output::default();

    /* skips invalid files */
    let contents = match read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(found),
    };

    for (i, line) in contents.lines().enumerate() {
        if matches(line, &cfg.pattern) {
            Output::push_match(
                &mut w,
                SearchResult::new(String::from(path), String::from(line), i + 1),
                cfg,
            );
            found = true;
        }
    }
    w.flush_to(&mut out);
    Ok(found)
}

pub fn matches(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

pub fn is_binary_heuristic(path: &str) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 1024];
    let bytes_read = file.read(&mut buffer)?;

    let has_null = buffer[..bytes_read].contains(&0);
    Ok(has_null)
}

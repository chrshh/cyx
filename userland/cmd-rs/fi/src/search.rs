use std::{
    fs::{metadata, read_dir, read_to_string},
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

    let found = if meta.is_dir() {
        search_dir(&cfg.path, cfg)?
    } else {
        search_file(&cfg.path, cfg)?
    };

    Ok(found)
}

/* iterates through entries and calls search_file when a file is found */
pub fn search_dir(path: &str, cfg: &Config) -> Result<bool, CGrepError> {
    let mut found = false;

    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.path().is_file() {
            found |= search_file(entry.path().to_str().unwrap(), cfg)?;
        }
    }
    Ok(found)
}

pub fn search_file(path: &str, cfg: &Config) -> Result<bool, CGrepError> {
    let contents = read_to_string(path)?;
    let mut found = false;

    for (i, line) in contents.lines().enumerate() {
        if matches(line, &cfg.pattern) {
            Output::print_match(
                SearchResult::new(String::from(path), String::from(line), i + 1),
                cfg,
            );
            found = true;
        }
    }
    Ok(found)
}

pub fn matches(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

use std::{
    fs::{File, metadata, read_dir, read_to_string},
    io::{self, BufWriter, Read, Write, stdout},
    path::Path,
    sync::{Arc, Mutex, atomic::Ordering},
    time::Instant,
};

use crate::{
    MATCH_FOUND, error::CGrepError, flags::RECURSIVE, parse::Config, thread::ThreadPool,
    writer::Output,
};

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
pub fn search(cfg: Arc<Config>, start: Instant) -> Result<bool, CGrepError> {
    if !Path::new(&cfg.path).exists() {
        return Err(CGrepError::NotFound(cfg.path.clone()));
    }
    let meta = metadata(&cfg.path)?;

    if meta.is_dir() && cfg.flags & RECURSIVE == 0 {
        return Err(CGrepError::IsDir(cfg.path.clone()));
    }

    let out = Arc::new(Mutex::new(BufWriter::new(stdout())));

    let found = if meta.is_dir() {
        let tp = ThreadPool::new()?;
        search_dir(&cfg.path, &Arc::clone(&cfg), &tp, &out)?
    } else {
        let mut guard = out.lock().unwrap();
        search_file(&cfg.path, &cfg, &mut *guard)?
    };

    /* print info that can only exist after execution of program is complete */
    let end = start.elapsed();
    let out_copy = Arc::clone(&out);
    let mut guard = out_copy.lock().unwrap();
    let mut w = Output::default();
    Output::post_search_output(&mut w, &cfg, end);
    Output::flush_to(&mut w, &mut *guard);

    Ok(found)
}

/* iterates through entries and calls search_file when a file is found */
pub fn search_dir<W: Write + Send + 'static>(
    path: &str,
    cfg: &Arc<Config>,
    tp: &ThreadPool,
    out: &Arc<Mutex<W>>,
) -> Result<bool, CGrepError> {
    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if skip_entry(&entry.path()) {
            continue;
        }
        if entry.path().is_file() {
            let entry = entry.path();
            let out_clone = Arc::clone(out);
            let cfg = Arc::clone(cfg);
            tp.execute(move || {
                let mut guard = out_clone.lock().unwrap();
                search_file(entry.to_str().unwrap(), &cfg, &mut *guard).unwrap();
            });
        } else {
            search_dir(entry.path().to_str().unwrap(), cfg, tp, out).unwrap();
        }
    }
    Ok(true)
}

pub fn search_file<W: Write>(path: &str, cfg: &Config, out: &mut W) -> Result<bool, CGrepError> {
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
        if matches(line, Config::as_ref(cfg)) {
            set_success_exit_code();
            Output::push_match(
                &mut w,
                SearchResult::new(String::from(path), String::from(line), i + 1),
                cfg,
            );
            found = true;
        }
    }
    w.flush_to(out);
    Ok(found)
}

/* returns true if condition is met to skip entry */
pub fn skip_entry(entry: &Path) -> bool {
    let skip = false;
    /* dotfile check */
    if skip | is_dotfile(entry) {
        return true;
    }
    /* rust built output check */
    if skip | is_rs_target_dir(entry) {
        return true;
    }
    /* node_modules dir check */
    if skip | is_node_mods_dir(entry) {
        return true;
    }
    skip
}

/* returns true if entry begins with '.' */
pub fn is_dotfile(entry: &Path) -> bool {
    let dotfile_count = entry
        .to_str()
        .unwrap()
        .split('/')
        .filter(|f| f.starts_with('.'))
        .count();
    dotfile_count > 0
}

/* returns true if entry is the rust build output directory */
pub fn is_rs_target_dir(entry: &Path) -> bool {
    let target_dir_count = entry
        .to_str()
        .unwrap()
        .split('/')
        .filter(|f| f.contains("target"))
        .count();
    target_dir_count > 0
}

pub fn is_node_mods_dir(entry: &Path) -> bool {
    let node_mods_dir_count = entry
        .to_str()
        .unwrap()
        .split('/')
        .filter(|f| f.contains("node_modules"))
        .count();
    node_mods_dir_count > 0
}

/* XXX extend this method to determine result based on input flags */
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

pub fn set_success_exit_code() {
    MATCH_FOUND.store(true, Ordering::Relaxed);
}

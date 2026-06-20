use std::{
    fs::{metadata, read_dir},
    io::{BufWriter, Write, stdout},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, atomic::Ordering},
    time::Instant,
};

use crate::{
    MATCH_FOUND, error::CFindError, flags::EntryTypeFilter, parse::Config, thread::ThreadPool,
    writer::Output,
};

#[derive(Debug)]
pub struct SearchResult {
    pub filepath: String,
    pub entry_type: EntryTypeFilter,
}

impl SearchResult {
    pub fn new(filepath: String, entry_type: Option<EntryTypeFilter>) -> Self {
        Self {
            filepath,
            entry_type: entry_type.unwrap(),
        }
    }
}

/* main entry point for searching */
pub fn search(cfg: Arc<Config>, start: Instant) -> Result<bool, CFindError> {
    /* if user provided a path, make sure it exists */
    if cfg.root_path.as_deref() != Some("") && !Path::new(&cfg.root_path.as_ref().unwrap()).exists()
    {
        return Err(CFindError::NotFound(
            cfg.root_path.as_ref().unwrap().clone(),
        ));
    }

    /* ensure user provided path is not a file */
    let meta = metadata(cfg.root_path.as_ref().unwrap())?;
    if meta.is_file() {
        return Err(CFindError::IsFile(cfg.root_path.as_ref().unwrap().clone()));
    }

    let out = Arc::new(Mutex::new(BufWriter::new(stdout())));
    let tp = ThreadPool::new()?;

    let skip_entry = false;

    for entry in read_dir(cfg.root_path.as_ref().unwrap()).unwrap() {
        let entry = entry.unwrap();
        if skip_entry {
            continue;
        }
        let entry = entry.path();
        let out_clone = Arc::clone(&out);
        let cfg = Arc::clone(&cfg);
        tp.execute(move || {
            let mut guard = out_clone.lock().unwrap();
            search_dirs(&cfg, &mut *guard, entry).unwrap();
        });
    }

    /* post search functions */
    let _ = start.elapsed();

    Ok(true)
}

pub fn search_dirs<W: Write>(
    cfg: &Arc<Config>,
    out: &mut W,
    input_entry: PathBuf,
) -> Result<bool, CFindError> {
    let found = false;
    let mut w = Output::default();

    for entry in read_dir(input_entry).unwrap() {
        let entry = entry.unwrap();

        /* recurse one level lower */
        if entry.path().is_dir() {
            search_dirs(cfg, out, entry.path())?;
        }

        /* check if non-dir entry matches */
        if check_match(entry.path().to_str().unwrap(), &cfg.clone()) {
            set_success_exit_code();
            Output::push_match(
                &mut w,
                SearchResult::new(entry.path().to_str().unwrap().to_string(), cfg.entry_type),
                cfg,
            );
        }
    }
    w.flush_to(out);

    Ok(found)
}

pub fn check_match(input_entry: &str, cfg: &Config) -> bool {
    input_entry.contains(&cfg.pattern.as_ref().unwrap().clone())
}

pub fn set_success_exit_code() {
    MATCH_FOUND.store(true, Ordering::Relaxed);
}

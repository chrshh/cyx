use std::{
    fs::{metadata, read_dir},
    io::{BufWriter, Write, stdout},
    os::unix::fs::PermissionsExt,
    path::Path,
    sync::{Arc, Mutex, atomic::Ordering},
    time::Instant,
};

use crate::{
    MATCH_FOUND,
    error::CFindError,
    flags::{ALL_FILES, ENTRY_TYPE_SET, EntryTypeFilter, PATTERN_IS_EXT},
    parse::Config,
    thread::ThreadPool,
    writer::Output,
};

#[derive(Debug)]
pub struct SearchResult {
    pub filepath: String,
}

impl SearchResult {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

/* main entry point for searching */
pub fn search(cfg: Arc<Config>, start: Instant) -> Result<bool, CFindError> {
    let root = cfg.root_path.as_deref().unwrap_or(".").to_string();

    if !Path::new(&root).exists() {
        return Err(CFindError::NotFound(root));
    }

    let meta = metadata(&root)?;
    if meta.is_file() {
        return Err(CFindError::IsFile(root));
    }

    let out = Arc::new(Mutex::new(BufWriter::new(stdout())));

    /* scope the pool so all queued jobs finish before post-search output */
    {
        let tp = ThreadPool::new()?;
        search_dir(&root, &Arc::clone(&cfg), &tp, &out);
    }

    let _ = start.elapsed();

    let out_copy = Arc::clone(&out);
    let mut guard = out_copy.lock().unwrap();
    let mut w = Output::default();
    Output::post_search_output(&mut w);
    w.flush_to(&mut *guard);

    Ok(MATCH_FOUND.load(Ordering::Relaxed))
}

/* walks directories synchronously; dispatches the per-entry match check
   to the thread pool so the directory traversal isn't blocked by output
   locking */
pub fn search_dir<W: Write + Send + 'static>(
    path: &str,
    cfg: &Arc<Config>,
    tp: &ThreadPool,
    out: &Arc<Mutex<W>>,
) {
    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let p = entry.path();

        if skip_entry(&p) {
            continue;
        }

        let out_clone = Arc::clone(out);
        let cfg_clone = Arc::clone(cfg);
        let p_clone = p.clone();
        tp.execute(move || {
            if check_match(&p_clone, &cfg_clone) {
                set_success_exit_code();
                let mut w = Output::default();
                Output::push_match(
                    &mut w,
                    SearchResult::new(p_clone.to_string_lossy().to_string()),
                    &cfg_clone,
                );
                let mut guard = out_clone.lock().unwrap();
                w.flush_to(&mut *guard);
            }
        });

        if p.is_dir() {
            search_dir(p.to_str().unwrap_or(""), cfg, tp, out);
        }
    }
}

pub fn check_match(path: &Path, cfg: &Config) -> bool {
    /* entry type filter (-t f/d/x) */
    if cfg.flags & ENTRY_TYPE_SET != 0
        && let Some(et) = cfg.entry_type
    {
        match et {
            EntryTypeFilter::File => {
                if !path.is_file() {
                    return false;
                }
            }
            EntryTypeFilter::Dir => {
                if !path.is_dir() {
                    return false;
                }
            }
            EntryTypeFilter::Exe => {
                if !path.is_file() {
                    return false;
                }
                match path.metadata() {
                    Ok(m) => {
                        if m.permissions().mode() & 0o111 == 0 {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }
            }
        }
    }

    /* no pattern -> everything that passed the type filter */
    if cfg.flags & ALL_FILES != 0 {
        return true;
    }

    let pattern = match cfg.pattern.as_deref() {
        Some(p) if !p.is_empty() => p,
        _ => return true,
    };

    /* -e: pattern is an exact file extension */
    if cfg.flags & PATTERN_IS_EXT != 0 {
        return path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e == pattern)
            .unwrap_or(false);
    }

    /* default: substring match on the file name only */
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.contains(pattern))
        .unwrap_or(false)
}

/* returns true if condition is met to skip entry */
pub fn skip_entry(entry: &Path) -> bool {
    is_dotfile(entry) || is_rs_target_dir(entry) || is_node_mods_dir(entry)
}

/* returns true if the entry's basename starts with '.' */
pub fn is_dotfile(entry: &Path) -> bool {
    entry
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false)
}

/* returns true if the entry is a rust build output directory */
pub fn is_rs_target_dir(entry: &Path) -> bool {
    entry
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "target")
        .unwrap_or(false)
}

pub fn is_node_mods_dir(entry: &Path) -> bool {
    entry
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "node_modules")
        .unwrap_or(false)
}

pub fn set_success_exit_code() {
    MATCH_FOUND.store(true, Ordering::Relaxed);
}

use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Instant;

pub mod error;
pub mod flags;
pub mod parse;
pub mod search;
pub mod thread;
pub mod writer;

pub use error::CFindError;
pub use parse::Config;

use crate::{
    parse::parse_args,
    search::{collect_matches, search},
};

pub static MATCH_FOUND: AtomicBool = AtomicBool::new(false);

/* binary entry point */
pub fn run(raw_args: &[String]) -> Result<bool, CFindError> {
    let start = Instant::now();
    let cfg = parse_args(raw_args)?;
    let cfg = Arc::new(cfg);
    search(cfg, start)
}

/* lib API entry point */
pub fn find_in(pattern: Option<&str>, path: &str, flags: u32) -> Result<Vec<PathBuf>, CFindError> {
    let mut flags = flags;
    if pattern.is_none() {
        flags |= flags::ALL_FILES;
    }
    let cfg = Config {
        pattern: pattern.map(|p| p.to_string()),
        root_path: Some(path.to_string()),
        entry_type: None,
        flags,
    };
    collect_matches(Arc::new(cfg))
}

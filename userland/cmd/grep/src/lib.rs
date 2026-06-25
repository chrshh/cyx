use std::sync::{Arc, atomic::AtomicBool};
use std::time::Instant;

pub mod error;
pub mod flags;
pub mod parse;
pub mod search;
pub mod thread;
pub mod writer;

pub use error::CGrepError;
pub use parse::Config;
pub use search::SearchResult;

use crate::flags::RECURSIVE;
use crate::{
    parse::parse_args,
    search::{collect_matches, search},
};

pub static MATCH_FOUND: AtomicBool = AtomicBool::new(false);

/* binary entry point */
pub fn run(raw_args: &[String]) -> Result<bool, CGrepError> {
    let start = Instant::now();
    let mut cfg = parse_args(raw_args)?;
    if cfg.flags == 0 {
        cfg.flags |= RECURSIVE;
    }
    let cfg = Arc::new(cfg);
    search(cfg, start)
}

/* lib API entry point */
pub fn search_in(pattern: &str, path: &str, flags: u32) -> Result<Vec<SearchResult>, CGrepError> {
    let mut cfg = Config {
        pattern: pattern.to_string(),
        path: path.to_string(),
        flags,
    };
    if cfg.flags == 0 {
        cfg.flags |= RECURSIVE;
    }
    collect_matches(Arc::new(cfg))
}

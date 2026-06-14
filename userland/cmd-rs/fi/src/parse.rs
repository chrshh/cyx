/* args[1] = FLAGS */
/* args[2] = needle */
/* args[3] = haystack */

use crate::{
    error::CGrepError,
    flags::{FlagConfig, parse_flags},
};

#[derive(Debug, Default)]
pub struct Config {
    pub pattern: String,
    pub path: String,
    pub flags: u32,
}

/* extracts out correct flags, pattern, and path based on positioning */
pub fn parse_args(raw_args: &[String]) -> Result<Config, CGrepError> {
    if raw_args.is_empty() {
        return Err(CGrepError::PatternMissing);
    }
    let flags = parse_flags(&raw_args[0])?;

    match flags {
        0 => {
            if let Some(n) = raw_args.get(FlagConfig::NoFlags.query_idx())
                && n.is_empty()
            {
                return Err(CGrepError::PatternMissing);
            }

            if let Some(h) = raw_args.get(FlagConfig::NoFlags.query_idx())
                && h.is_empty()
            {
                return Err(CGrepError::PathMissing);
            }

            Ok(Config {
                pattern: raw_args[FlagConfig::NoFlags.query_idx()].clone(),
                path: raw_args[FlagConfig::NoFlags.path_idx()].clone(),
                flags,
            })
        }
        _ => {
            if let Some(n) = raw_args.get(FlagConfig::Flags.query_idx())
                && n.is_empty()
            {
                return Err(CGrepError::PatternMissing);
            }
            if let Some(h) = raw_args.get(FlagConfig::Flags.path_idx())
                && h.is_empty()
            {
                return Err(CGrepError::PathMissing);
            }

            Ok(Config {
                pattern: raw_args[FlagConfig::Flags.query_idx()].clone(),
                path: raw_args[FlagConfig::Flags.path_idx()].clone(),
                flags,
            })
        }
    }
}

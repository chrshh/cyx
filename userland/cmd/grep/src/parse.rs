/* args[1] = FLAGS */
/* args[2] = needle */
/* args[3] = haystack */

use crate::{
    error::CGrepError,
    flags::{FlagConfig, parse_flags},
};

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub pattern: String,
    pub path: String,
    pub flags: u32,
}

impl AsRef<str> for Config {
    fn as_ref(&self) -> &str {
        &self.pattern
    }
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

            /* if no path is provided, cwd is used by default */
            let path = match raw_args.get(FlagConfig::NoFlags.path_idx()) {
                Some(h) if !h.is_empty() => h.clone(),
                _ => std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            };

            Ok(Config {
                pattern: raw_args[FlagConfig::NoFlags.query_idx()].clone(),
                path,
                flags,
            })
        }
        _ => {
            if let Some(n) = raw_args.get(FlagConfig::Flags.query_idx())
                && n.is_empty()
            {
                return Err(CGrepError::PatternMissing);
            }

            /* if no path is provided, cwd is used by default */
            let path = match raw_args.get(FlagConfig::Flags.path_idx()) {
                Some(h) if !h.is_empty() => h.clone(),
                _ => std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            };

            Ok(Config {
                pattern: raw_args[FlagConfig::Flags.query_idx()].clone(),
                path,
                flags,
            })
        }
    }
}

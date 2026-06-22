use crate::{
    error::CFindError,
    flags::{ALL_FILES, EntryTypeFilter, parse_flags},
};

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub pattern: Option<String>,
    pub root_path: Option<String>,
    pub entry_type: Option<EntryTypeFilter>,
    pub flags: u32,
}

pub fn parse_args(raw_args: &[String]) -> Result<Config, CFindError> {
    /* EXAMPLES */
    // cfd                   -> list everything from CWD
    // cfd <pattern>         -> search CWD for entries whose name matches
    // cfd <pattern> <path>  -> search <path> instead
    // cfd -e log            -> all *.log entries under CWD
    // cfd -t f <pattern>    -> only files
    // cfd -t d <pattern>    -> only directories
    // cfd -t x <pattern>    -> only executables

    let mut cfg = Config::default();

    /* zero-arg form: list everything under CWD */
    if raw_args.is_empty() {
        cfg.flags |= ALL_FILES;
        cfg.root_path = Some(String::from("."));
        return Ok(cfg);
    }

    let positional = parse_flags(&mut cfg, raw_args)?;

    /* If a flag (-e) already consumed the pattern, the next positional
       is the optional root path. Otherwise the first positional is the
       pattern and the second is the optional root path. */
    let mut iter = positional.into_iter();
    if cfg.pattern.is_none() {
        if let Some(p) = iter.next() {
            cfg.pattern = Some(p);
        }
    }
    if let Some(rp) = iter.next() {
        cfg.root_path = Some(rp);
    }

    if cfg.root_path.is_none() {
        cfg.root_path = Some(String::from("."));
    }

    /* no pattern + no -e -> list everything that passes the type filter */
    if cfg.pattern.is_none() {
        cfg.flags |= ALL_FILES;
    }

    Ok(cfg)
}

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
    // cfd
    // cfd <pattern>
    // cfd -e log
    // cfd -t <t> <pattern>
    //
    if raw_args.is_empty() {
        return Err(CFindError::PatternMissing);
    }

    let mut cfg = Config::default();
    let args = parse_flags(&mut cfg, raw_args)?;

    /* default root_path of current dir */
    if cfg.root_path.is_none() {
        cfg.root_path = Some(String::from("."));
    }

    /* only 'cfd' was entered  */
    if cfg.flags & ALL_FILES != 0 {
        return Ok(cfg);
    }

    /* 'cfd + <pattern> + OPTIONAL<root_path>' */
    if cfg.pattern.as_deref() == Some("") {
        cfg.pattern = Some(args[0].clone());
        if !args[1].is_empty() {
            cfg.root_path = Some(args[1].clone());
        }
        return Ok(cfg);
    }

    println!("entry type: {:?}", cfg.entry_type);
    println!("pattern from ext: {:?}", cfg.pattern);

    Ok(cfg)
}

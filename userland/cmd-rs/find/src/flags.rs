use crate::{error::CFindError, parse::Config};

pub const ALL_FILES: u32 = 1 << 0; // flag is only set when 0 cli args are presented
pub const PATTERN_IS_EXT: u32 = 1 << 1; // flag is set when search pattern is a file extension
pub const ENTRY_TYPE_SET: u32 = 1 << 2;

#[derive(Debug, Clone, Copy)]
pub enum EntryTypeFilter {
    File,
    Dir,
    Exe,
}

pub fn parse_flags(cfg: &mut Config, raw_args: &[String]) -> Result<Vec<String>, CFindError> {
    /* only cfd was given as input, all files will be listed now without a search pattern */
    if raw_args[0].is_empty() {
        cfg.flags |= ALL_FILES;
        return Ok(Vec::new());
    };

    if raw_args[0].starts_with('-') {
        for c in raw_args[0].chars() {
            match c {
                /* Entry Type flag */
                't' => {
                    /* Entry type param check */
                    if raw_args[1].is_empty() {
                        return Err(CFindError::UnkownEntryType(String::from("NULL")));
                    }

                    /* assign type filter */
                    for ch in raw_args[1].chars() {
                        match ch {
                            'f' => cfg.entry_type = Some(EntryTypeFilter::File),
                            'd' => cfg.entry_type = Some(EntryTypeFilter::Dir),
                            'x' => cfg.entry_type = Some(EntryTypeFilter::Exe),
                            _ => return Err(CFindError::MissingFlag),
                        }
                    }
                    cfg.flags |= ENTRY_TYPE_SET;
                    return Ok(raw_args[2..].to_vec());
                }
                /* File Extension */
                'e' => {
                    if raw_args[1].is_empty() {
                        return Err(CFindError::PatternMissing);
                    }

                    cfg.pattern = Some(raw_args[1].clone());
                    cfg.flags |= PATTERN_IS_EXT;
                    return Ok(raw_args[2..].to_vec());
                }
                '-' => continue,
                other => return Err(CFindError::UnknownFlag(other.to_string())),
            }
        }
    }

    Ok(raw_args.to_vec())
}

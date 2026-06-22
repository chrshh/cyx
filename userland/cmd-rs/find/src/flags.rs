use crate::{error::CFindError, parse::Config};

pub const ALL_FILES: u32 = 1 << 0; // no pattern was supplied
pub const PATTERN_IS_EXT: u32 = 1 << 1; // pattern came from -e (file extension)
pub const ENTRY_TYPE_SET: u32 = 1 << 2;

#[derive(Debug, Clone, Copy)]
pub enum EntryTypeFilter {
    File,
    Dir,
    Exe,
}

pub fn parse_flags(cfg: &mut Config, raw_args: &[String]) -> Result<Vec<String>, CFindError> {
    if !raw_args[0].starts_with('-') {
        return Ok(raw_args.to_vec());
    }

    for c in raw_args[0].chars() {
        match c {
            '-' => continue,
            /* Entry Type flag */
            't' => {
                let kind = raw_args
                    .get(1)
                    .ok_or_else(|| CFindError::UnkownEntryType(String::from("NULL")))?;

                for ch in kind.chars() {
                    match ch {
                        'f' => cfg.entry_type = Some(EntryTypeFilter::File),
                        'd' => cfg.entry_type = Some(EntryTypeFilter::Dir),
                        'x' => cfg.entry_type = Some(EntryTypeFilter::Exe),
                        _ => return Err(CFindError::UnkownEntryType(ch.to_string())),
                    }
                }
                cfg.flags |= ENTRY_TYPE_SET;
                return Ok(raw_args[2..].to_vec());
            }
            /* File Extension */
            'e' => {
                let ext = raw_args.get(1).ok_or(CFindError::PatternMissing)?;
                cfg.pattern = Some(ext.clone());
                cfg.flags |= PATTERN_IS_EXT;
                return Ok(raw_args[2..].to_vec());
            }
            other => return Err(CFindError::UnknownFlag(other.to_string())),
        }
    }

    Ok(raw_args.to_vec())
}

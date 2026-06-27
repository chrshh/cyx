use std::path::PathBuf;

#[derive(Debug)]
pub struct RunConfig {
    pub provided_dir: PathBuf,
}

pub const PICKER_FLAG_MODE: &str = "--pick-file";
pub const CWD: &str = "--cwd";

pub fn parse_args(raw_args: Vec<String>) -> Option<RunConfig> {
    let args = &raw_args[1..];

    /* not in picker mode -> fallthrough */
    if args.first().map(String::as_str) != Some(PICKER_FLAG_MODE) {
        return None;
    }

    /* <cwd> is optional -> treat empty as use cwd */
    let provided_dir = match args.get(1).map(String::as_str) {
        Some(CWD) => PathBuf::from(args.get(2).cloned().unwrap()),
        _ => PathBuf::new(),
    };

    Some(RunConfig { provided_dir })
}

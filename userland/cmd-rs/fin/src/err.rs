#[derive()]
pub enum FError<S> {
    PatternMissing,
    PathMissing,
    FileNotFound(S),
    UnknownFlag(char),
}

impl<S: AsRef<str>> FError<S> {
    pub fn exit(self) -> ! {
        let err_msg = match self {
            FError::PatternMissing => "missing pattern".to_string(),
            FError::PathMissing => "missing path".to_string(),
            FError::FileNotFound(s) => format!("cannot open '{:?}': no such file", s.as_ref()),
            FError::UnknownFlag(c) => format!("unknown flag: '{c}'"),
        };
        eprintln!("find: {}", err_msg);
        std::process::exit(1);
    }

    pub fn call_exit(self) -> ! {
        Self::exit(self);
    }
}

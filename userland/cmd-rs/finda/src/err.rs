#[derive()]
pub enum GrError<S> {
    PatternMissing,
    PathMissing,
    FileNotFound(S),
    UnknownFlag(char),
    IsDir(S),
}

impl<S: AsRef<str>> GrError<S> {
    pub fn exit(self) -> ! {
        let err_msg = match self {
            GrError::PatternMissing => "missing pattern".to_string(),
            GrError::PathMissing => "missing path".to_string(),
            GrError::FileNotFound(s) => format!("cannot open '{:?}': no such file", s.as_ref()),
            GrError::UnknownFlag(c) => format!("unknown flag: '{c}'"),
            GrError::IsDir(s) => format!("'{:?}' is a directory", s.as_ref()),
        };
        eprintln!("find: {}", err_msg);
        std::process::exit(1);
    }

    pub fn call_exit(self) -> ! {
        Self::exit(self);
    }
}

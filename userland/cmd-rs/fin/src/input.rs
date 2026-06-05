use std::env::args;

use crate::err::FError;

const IGNORE_CASE: u32 = 1 << 0;
const RECURSIVE: u32 = 1 << 1;
const WHOLE_WORD: u32 = 1 << 2;
const LN_NUMS: u32 = 1 << 3;
const COUNT: u32 = 1 << 4;

pub struct InputArgs {}

enum Config {
    Flags,
    NoFlags,
}

impl Config {
    fn query_idx(&self) -> usize {
        match self {
            Config::NoFlags => 1,
            Config::Flags => 2,
        }
    }

    fn path_idx(&self) -> usize {
        match self {
            Config::NoFlags => 2,
            Config::Flags => 3,
        }
    }
}

#[derive(Clone)]
pub struct OutputArgs {
    pub flags: u32,
    pub query: String,
    pub path: String,
}

impl InputArgs {
    pub fn parse_args(v: Vec<String>) -> OutputArgs {
        if v.len() < 2 {
            FError::call_exit(FError::<&str>::PatternMissing);
        }
        match Self::parse_flags(&v[1]) {
            Ok(0) => Self::parse_without_flags(v),
            Ok(flag) => Self::parse_with_flags(flag, v),
            Err(e) => FError::call_exit(e),
        }
    }
    pub fn parse_flags(f: &str) -> Result<u32, FError<&str>> {
        if !f.starts_with('-') {
            return Ok(0);
        };

        let mut flag: u32 = 0;
        for c in f.chars() {
            match c {
                '-' => {}
                'i' => flag |= IGNORE_CASE,
                'r' => flag |= RECURSIVE,
                'w' => flag |= WHOLE_WORD,
                'n' => flag |= LN_NUMS,
                'c' => flag |= COUNT,
                other => return Err(FError::UnknownFlag(other)),
            }
        }

        Ok(flag)
    }

    fn parse_with_flags(f: u32, v: Vec<String>) -> OutputArgs {
        // Null Query check
        if let Some(q) = v.get(Config::Flags.query_idx())
            && q.is_empty()
        {
            FError::call_exit(FError::<&str>::PatternMissing);
        }

        // Null Path check
        if let Some(p) = v.get(Config::Flags.path_idx())
            && p.is_empty()
        {
            FError::call_exit(FError::<&str>::PathMissing)
        }
        OutputArgs {
            flags: f,
            query: v[Config::Flags.query_idx()].clone(),
            path: v[Config::Flags.path_idx()].clone(),
        }
    }

    fn parse_without_flags(v: Vec<String>) -> OutputArgs {
        if let Some(q) = v.get(Config::NoFlags.query_idx())
            && q.is_empty()
        {
            FError::call_exit(FError::<&str>::PatternMissing);
        }

        // Null Path check
        if let Some(p) = v.get(Config::NoFlags.path_idx())
            && p.is_empty()
        {
            FError::call_exit(FError::<&str>::PathMissing)
        }

        OutputArgs {
            flags: 0,
            query: v[Config::NoFlags.query_idx()].clone(),
            path: v[Config::NoFlags.path_idx()].clone(),
        }
    }
}

pub fn get_args() -> Vec<String> {
    args().collect()
}

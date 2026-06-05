use std::{env::args, ops::BitAnd};

const IGNORE_CASE: u32 = 1 << 0;
const RECURSIVE: u32 = 1 << 1;
const WHOLE_WORD: u32 = 1 << 2;
const LN_NUMS: u32 = 1 << 3;
const COUNT: u32 = 1 << 4;
const INVALID: u32 = 1 << 31;

// parse_args<no_flags>

enum FError<S> {
    PatternMissing,
    PathMissing,
    FileNotFound(S),
    UnknownFlag(S),
}

impl<S: AsRef<str>> FError<S>
where
    S: std::fmt::Debug,
{
    fn exit(self) -> ! {
        let err_msg = match self {
            FError::PatternMissing => "missing pattern".to_string(),
            FError::PathMissing => "missing path".to_string(),
            FError::FileNotFound(s) => format!("cannot open '{:?}': no such file", s.as_ref()),
            FError::UnknownFlag(s) => format!("unknown flag: '{:?}'", s.as_ref()),
        };
        eprintln!("find: {}", err_msg);
        std::process::exit(1);
    }

    fn call_exit(self) {
        Self::exit(self);
    }
}

struct InputArgs;

#[derive(Clone)]
struct OutputArgs {
    flags: u32,
    query: String,
    path: String,
}

impl InputArgs {
    fn parse_args(v: Vec<String>) -> OutputArgs {
        let mut f = String::new();
        v[1].clone_into(&mut f);
        let temp = Self::parse_flags(&mut f);

        // Valid flag config
        if let Ok(flag) = temp
            && flag != 0
        {
            // Null Query check
            if let Some(q) = v.get(2)
                && q.is_empty()
            {
                FError::call_exit(FError::<&str>::PatternMissing);
            }

            // Null Path check
            if let Some(p) = v.get(3)
                && p.is_empty()
            {
                FError::call_exit(FError::<&str>::PathMissing)
            }
            OutputArgs {
                flags: flag,
                query: v[2].clone(),
                path: v[3].clone(),
            }
        } else {
            if let Some(q) = v.get(1)
                && q.is_empty()
            {
                FError::call_exit(FError::<&str>::PatternMissing);
            }

            // Null Path check
            if let Some(p) = v.get(2)
                && p.is_empty()
            {
                FError::call_exit(FError::<&str>::PathMissing)
            }

            OutputArgs {
                flags: 0,
                query: v[1].clone(),
                path: v[2].clone(),
            }
        }
    }

    fn parse_flags(f: &mut str) -> Result<u32, FError<&str>> {
        if !f.starts_with('-') {
            return Ok(0);
        };

        let mut flag: u32 = 0;
        for c in f.chars() {
            match c {
                'i' => flag |= IGNORE_CASE,
                'r' => flag |= RECURSIVE,
                'w' => flag |= WHOLE_WORD,
                'n' => flag |= LN_NUMS,
                'c' => flag |= COUNT,
                _ => flag |= INVALID,
            }
        }

        if flag.bitand(INVALID) == 1 {
            return Err(FError::UnknownFlag("unknown flag"));
        }

        Ok(flag)
    }
}

fn get_args() -> Vec<String> {
    args().collect()
}

fn main() {
    let args = get_args();
    /* args[1] = FLAGS */
    /* args[2] = QUERY */
    /* args[3] = FILEPATH */

    if args.len() < 2 {
        FError::<&str>::PatternMissing.exit();
    }

    let argv = InputArgs::parse_args(args);

    println!("{}", argv.flags);
    println!("{}", argv.query);
    println!("{}", argv.path);
}

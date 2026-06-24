use std::path::{Path, PathBuf};

pub trait PathExt {
    fn pretty(&self) -> String;
}

impl PathExt for Path {
    fn pretty(&self) -> String {
        let prefix = "~/";
        let rel_path: PathBuf = self.components().skip(3).collect();
        prefix.to_string() + rel_path.to_str().unwrap()
    }
}

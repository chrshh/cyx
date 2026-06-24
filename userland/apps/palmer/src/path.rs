use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub trait PathExt {
    fn pretty(&self) -> String;
}

pub trait PathBufExt {
    fn get_all(&self) -> Vec<String>;
}

impl PathExt for Path {
    fn pretty(&self) -> String {
        let prefix = "~/";
        let rel_path: PathBuf = self.components().skip(3).collect();
        prefix.to_string() + rel_path.to_str().unwrap()
    }
}

impl PathBufExt for PathBuf {
    fn get_all(&self) -> Vec<String>
    where
        Self: std::convert::AsRef<std::path::Path>,
    {
        let entries: Vec<String> = read_dir(self)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();

        entries
    }
}

use std::path::{Path, PathBuf};

pub struct Config {
    pub site_root: PathBuf,
    pub cache_path: PathBuf,
    pub theme_path: PathBuf,
    pub hostname: String,
    pub port: u16
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site_root: Path::new("/mnt/c/Users/comet/Documents/Code/math").into(),
            cache_path: Path::new("/mnt/c/Users/comet/Documents/Code/math/.hecto").into(),
            theme_path: Path::new("themes/default").into(),
            hostname: "127.0.0.1".into(),
            port: 7878
        }
    }
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}
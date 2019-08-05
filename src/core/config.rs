use std::path::{Path, PathBuf};

pub struct Config {
    pub site_root: PathBuf,
    pub cache_path: PathBuf,
    pub theme_path: PathBuf,
    pub hostname: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site_root: Path::new("/Users/felixleveille/Code/uni/phy").into(),
            cache_path: Path::new("/Users/felixleveille/Code/uni/phy/.hecto").into(),
            theme_path: Path::new("themes/default").into(),
            hostname: "127.0.0.1".into(),
            port: 7878,
        }
    }
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

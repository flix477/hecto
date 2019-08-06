use std::path::{Path, PathBuf};
use std::env::{current_dir, current_exe};
use clap::ArgMatches;

pub struct Config {
    pub site_root: PathBuf,
    pub theme_path: PathBuf,
    pub hostname: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site_root: current_dir().unwrap(),
            theme_path: current_exe().unwrap().join(Path::new("themes/default").canonicalize().unwrap()),
            hostname: "127.0.0.1".into(),
            port: 7878,
        }
    }
}

impl From<ArgMatches<'_>> for Config {
    fn from(matches: ArgMatches) -> Self {
        let default = Config::default();
        let create_path = |value| Path::new(value).into();

        let site_root = matches.value_of("root")
            .map(create_path)
            .unwrap_or(default.site_root);

        let theme_path = matches.value_of("theme")
            .map(create_path)
            .unwrap_or(default.theme_path);

        Self {
            site_root,
            theme_path,
            hostname: matches.value_of("hostname").unwrap().into(),
            port: matches.value_of("port").unwrap()
                .parse()
                .expect("Invalid port number"),
        }
    }
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

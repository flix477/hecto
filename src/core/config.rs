use std::path::{Path, PathBuf};
use std::env::{current_dir, current_exe};
use clap::ArgMatches;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct Config {
    pub site_root: PathBuf,
    pub theme_path: PathBuf,
    pub address: SocketAddr
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site_root: current_dir().unwrap(),
            theme_path: current_exe().unwrap().join(Path::new("themes/default").canonicalize().unwrap()),
            address: ([127, 0, 0, 1], 7878).into()
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

        let hostname = matches.value_of("hostname").unwrap();
        let port = matches.value_of("port").unwrap();

        Self {
            site_root,
            theme_path,
            address: SocketAddr::from_str(&format!("{}:{}", hostname, port))
                .expect("Invalid host address")
        }
    }
}

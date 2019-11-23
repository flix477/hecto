#[macro_use]
extern crate clap;

use crate::core::config::Config;
use crate::core::parser::parse_folder;
use crate::core::posts::folders::list;
use crate::core::Hecto;
use crate::renderer::Renderer;
use clap::{App, Arg};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub mod core;
pub mod renderer;
pub mod server;
pub mod util;
pub mod watcher;

fn main() {
    let default_config = Config::default();
    let default_hostname = default_config.address.ip().to_string();
    let default_port = default_config.address.port().to_string();
    let matches = App::new("hecto")
        .version(crate_version!())
        .about("Super simple markdown blog server.")
        .arg(
            Arg::with_name("root")
                .help("Path to the root of the blog. Defaults to the current directory."),
        )
        .arg(
            Arg::with_name("hostname")
                .help("Hostname to host the blog.")
                .short("h")
                .long("hostname")
                .default_value(&default_hostname)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .help("Port to host the blog.")
                .short("p")
                .long("port")
                .default_value(&default_port)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("theme")
                .help("Path to the blog's theme. Defaults to Hecto's default theme.")
                .long("theme")
                .short("t")
                .value_name("PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .help("Path to the blog's configuration file.")
                .long("config")
                .short("c")
                .takes_value(true)
                .value_name("PATH"),
        )
        .get_matches();

    let config = Config::from(matches);
    let renderer = Renderer::new(&config);
    let root = parse_folder(&config.site_root, &renderer).expect("Error reading site root");
    dbg!(list(&root, Path::new("")));

    let state = Arc::new(Mutex::new(Hecto {
        config,
        root,
        renderer,
    }));

    if let Err(_) = watcher::initialize(state.clone()) {
        println!("Could not initialize hot-reloading. Changes to your blog will only appear after restarting Hecto.")
    }

    server::run(state.clone());
}

use std::path::Path;
#[macro_use]
extern crate lazy_static;
use crate::core::config::Config;
use crate::core::parser::parse_folder;
use crate::core::posts::folders::list;
use crate::core::Hecto;
use crate::renderer::Renderer;
use crate::server::Server;
use std::sync::{Arc, Mutex};

pub mod core;
pub mod renderer;
pub mod server;
pub mod util;
pub mod watcher;

fn main() {
    let config = Config::default();
    let renderer = Renderer::new(&config.theme_path);
    let root = parse_folder(&config.site_root, &renderer).expect("Error reading site root");
    dbg!(list(&root, Path::new("")));

    let state = Arc::new(Mutex::new(Hecto {
        config,
        root,
        renderer,
    }));

    if let Err(_) = watcher::initialize(state.clone()) {
        println!("Could not initialize auto-refresh. You should restart the server when you make changes to your site.")
    }

    Server::default().run(state.clone());
}

use std::path::Path;
#[macro_use]
extern crate lazy_static;
use crate::core::config::Config;
use crate::core::parser::parse_folder;
use crate::server::Server;
use crate::core::Hecto;
use crate::renderer::Renderer;

pub mod core;
pub mod renderer;
pub mod server;

fn main() {
    let config = Config::default();
    let renderer = Renderer::new(&config.theme_path);
    let root = parse_folder(&config.site_root, &renderer).expect("Error reading site root");
    dbg!(root.list(Path::new("")));
    Server::default().run(Hecto {
        config,
        root,
    });
}
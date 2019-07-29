use crate::core::config::Config;
use crate::core::folder::Folder;

pub mod config;
pub mod folder;
pub mod parser;
pub mod post;

pub struct Hecto {
    pub config: Config,
    pub root: Folder,
}
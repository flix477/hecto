use crate::core::config::Config;
use crate::core::folder::{Folder, FolderEntry};
use crate::core::post::Post;
use crate::renderer::Renderer;
use std::path::Path;
use std::error::Error;
use crate::core::parser::post::parse_post;

pub mod config;
pub mod folder;
pub mod parser;
pub mod post;

pub struct Hecto {
    pub config: Config,
    pub root: Folder,
    pub renderer: Renderer,
}

impl Hecto {
    pub fn element_at_path(&self, path: &Path) -> Option<FolderEntry<&Post, &Folder>> {
        if path.components().count() == 0 {
            Some(FolderEntry::Folder(&self.root))
        } else {
            self.root.element_at_path(path)
        }
    }

    pub fn rerender(&mut self) {
        self.root.rerender(&self.renderer);
    }

    pub fn rerender_posts(&mut self) {
        self.root.rerender_posts(&self.renderer);
    }

    pub fn render_post(&self, path: &Path) -> Result<Post, Box<dyn Error>> {
        parse_post(path, &self.renderer)
    }

    pub fn update_theme(&mut self) {
        self.renderer.register_templates(&self.config.theme_path);
    }
}

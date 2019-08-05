use crate::core::config::Config;
use crate::core::posts::folder::Folder;
use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::Post;
use crate::renderer::Renderer;
use std::path::Path;
use crate::core::posts::folders::element_at;

pub mod config;
pub mod parser;
pub mod posts;

pub struct Hecto {
    pub config: Config,
    pub root: Folder,
    pub renderer: Renderer,
}

impl Hecto {
    pub fn element_at_path(&self, path: &Path) -> Option<FolderEntry<&Post, &Folder>> {
        element_at(&self.root, path)
    }

    pub fn rerender(&mut self) {
        self.root.rerender(&self.renderer);
    }

    pub fn rerender_posts(&mut self) {
        self.root.rerender_posts(&self.renderer);
    }

    pub fn update_theme(&mut self) {
        self.renderer.register_templates(&self.config.theme_path);
    }
}

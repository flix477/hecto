use crate::core::posts::Post;
use crate::util::boxed_error;
use handlebars::Handlebars;
use std::error::Error;
use std::path::{Path, PathBuf};
use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::folder::Folder;

mod folder_view;
mod post_view;

pub struct Renderer {
    registry: Handlebars,
}

impl Renderer {
    pub fn new(theme_path: &PathBuf) -> Self {
        let mut registry = Handlebars::new();
        registry.set_strict_mode(true);
        let mut renderer = Renderer { registry };
        renderer.register_templates(&theme_path);
        renderer
    }

    pub fn render_post(&self, post: &Post) -> Result<String, Box<dyn Error>> {
        let view = post_view::PostView::new(post);
        self.registry
            .render(
                "post",
                &view,
            )
            .map_err(boxed_error)
    }

    pub fn render_folder(&self, folder: &Folder) -> Result<String, Box<dyn Error>> {
        let view = folder_view::FolderView::new(folder);
        self.registry
            .render(
                "folder",
                &view
            )
            .map_err(boxed_error)
    }

    pub fn register_templates(&mut self, theme_path: &PathBuf) {
        self.register_template("page", theme_path);
        self.register_template("post", theme_path);
        self.register_template("folder", theme_path);
    }

    fn register_template(&mut self, template: &str, theme_path: &PathBuf) {
        self.registry
            .register_template_file(
                template,
                path_with_component(&theme_path, format!("{}.tpl", template).as_str()),
            )
            .expect(format!("Cannot read template: {}", template).as_str());
    }
}

fn path_with_component(path: &Path, component: &str) -> PathBuf {
    path.join(Path::new(component))
}

pub trait ToHtml {
    fn to_html(&self, renderer: &Renderer) -> Result<String, Box<dyn Error>>;
}

impl ToHtml for FolderEntry<&Post, &Folder> {
    fn to_html(&self, renderer: &Renderer) -> Result<String, Box<dyn Error>> {
        match self {
            FolderEntry::Post(post) => renderer.render_post(post),
            FolderEntry::Folder(folder) => renderer.render_folder(folder),
        }
    }
}
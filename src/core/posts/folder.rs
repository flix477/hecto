use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::folders::{mut_element_at, mut_posts};
use crate::core::posts::Post;
use crate::renderer::Renderer;
use std::path::Path;

/// Represents a folder of posts or other folders
#[derive(Clone, Debug, Default)]
pub struct Folder {
    pub name: String,
    pub entries: Vec<FolderEntry<Post, Folder>>,
}

impl Folder {
    pub fn mut_folder_at_path(&mut self, path: &Path) -> Option<&mut Folder> {
        if path.is_file() {
            return None;
        }

        if let Some(FolderEntry::Folder(folder)) = mut_element_at(self, path) {
            Some(folder)
        } else {
            None
        }
    }

    pub fn rerender(&mut self, renderer: &Renderer) {
        // TODO: rerender folders
        self.rerender_posts(renderer);
    }

    pub fn rerender_posts(&mut self, renderer: &Renderer) {
        mut_posts(self.entries.iter_mut()).for_each(|mut post| match renderer.render_post(&post) {
            Ok(contents) => post.contents = contents,
            Err(error) => println!("Error rerendering post: {}", error.description()),
        });
    }
}

use crate::core::post::Post;
use crate::renderer::Renderer;
use std::path::{Path, PathBuf};
use crate::util::{first_component, component_as_string};

/// Represents a folder of posts or other folders
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub entries: Vec<FolderEntry<Post, Folder>>,
}

impl Into<FolderEntry<Post, Folder>> for Folder {
    fn into(self) -> FolderEntry<Post, Folder> {
        FolderEntry::Folder(self)
    }
}

impl Folder {
    pub fn mut_posts(&mut self) -> impl Iterator<Item = &mut Post> {
        self.entries.iter_mut().filter_map(|entry| {
            if let FolderEntry::Post(post) = entry {
                Some(post)
            } else {
                None
            }
        })
    }

    pub fn filter_folders<P, F>(entry: &FolderEntry<P, F>) -> Option<&F> {
        if let FolderEntry::Folder(ref folder) = entry {
            Some(folder)
        } else {
            None
        }
    }

    // TODO: i hate that this exists
    pub fn mut_filter_folders<P, F>(entry: &mut FolderEntry<P, F>) -> Option<&mut F> {
        if let FolderEntry::Folder(ref mut folder) = entry {
            Some(folder)
        } else {
            None
        }
    }

    pub fn recursive_posts(&self) -> Vec<&Post> {
        self.entries
            .iter()
            .flat_map(|entry| match entry {
                FolderEntry::Folder(folder) => folder.recursive_posts(),
                FolderEntry::Post(post) => vec![post],
            })
            .collect()
    }

    pub fn element_at_path(&self, path: &Path) -> Option<FolderEntry<&Post, &Folder>> {
        let first_component = first_component(path);

        if path.components().count() == 1 {
            self.entries
                .iter()
                .find(|post| post.name() == first_component)
                .map(|x| x.as_ref())
        } else {
            let folder = self.entries.iter()
                .filter_map(|entry| Self::filter_folders(entry))
                .find(|folder| folder.name == first_component)?;
            folder.element_at_path(Path::new(
                &path
                    .components()
                    .skip(1)
                    .map(component_as_string)
                    .collect::<Vec<String>>()
                    .join("/"),
            ))
        }
    }

    pub fn mut_element_at_path(&mut self, path: &Path) -> Option<FolderEntry<&mut Post, &mut Folder>> {
        let first_component = first_component(path);

        if path.components().count() == 1 {
            self.entries
                .iter_mut()
                .find(|entry| entry.name() == first_component)
                .map(|x| x.as_mut_ref())
        } else {
            let folder: &mut Folder = self.entries.iter_mut()
                .filter_map(Self::mut_filter_folders)
                .find(|folder| folder.name == first_component)?;
            folder.mut_element_at_path(Path::new(
                &path
                    .components()
                    .skip(1)
                    .map(component_as_string)
                    .collect::<Vec<String>>()
                    .join("/"),
            ))
        }
    }

    pub fn mut_folder_at_path(&mut self, path: &Path) -> Option<&mut Folder> {
        if path.is_file() { return None; }

        if let Some(FolderEntry::Folder(folder)) = self.mut_element_at_path(path) {
            Some(folder)
        } else {
            None
        }
    }

    pub fn list(&self, base_path: &Path) -> Vec<PathBuf> {
        self.entries
            .iter()
            .flat_map(|entry| match entry {
                FolderEntry::Folder(folder) => folder.list(&base_path.join(&folder.name)),
                FolderEntry::Post(post) => vec![base_path.join(&post.name)],
            })
            .collect()
    }

    pub fn rerender(&mut self, renderer: &Renderer) {
        // TODO: rerender folders
        self.rerender_posts(renderer);
    }

    pub fn rerender_posts(&mut self, renderer: &Renderer) {
        self.mut_posts()
            .for_each(|mut post| match renderer.render_post(&post) {
                Ok(contents) => post.contents = contents,
                Err(error) => println!("Error rerendering post: {}", error.description()),
            });
    }
}

#[derive(Clone, Debug)]
pub enum FolderEntry<P, F> {
    Post(P),
    Folder(F),
}

impl<P, F> FolderEntry<P, F> {
    pub fn as_ref(&self) -> FolderEntry<&P, &F> {
        match *self {
            FolderEntry::Post(ref post) => FolderEntry::Post(post),
            FolderEntry::Folder(ref folder) => FolderEntry::Folder(folder),
        }
    }

    pub fn as_mut_ref(&mut self) -> FolderEntry<&mut P, &mut F> {
        match *self {
            FolderEntry::Post(ref mut post) => FolderEntry::Post(post),
            FolderEntry::Folder(ref mut folder) => FolderEntry::Folder(folder),
        }
    }
}

impl FolderEntry<Post, Folder> {
    pub fn name(&self) -> String {
        match self {
            FolderEntry::Post(post) => post.name.clone(),
            FolderEntry::Folder(folder) => folder.name.clone(),
        }
    }
}

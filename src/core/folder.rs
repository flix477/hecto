use std::path::{Path, PathBuf};
use crate::core::post::Post;

/// Represents a folder of posts or other folders
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub entries: Vec<FolderEntry>
}

impl Folder {
    pub fn folders(&self) -> impl Iterator<Item = &Folder> {
        self.entries.iter()
            .filter_map(|entry| if let FolderEntry::Folder(folder) = entry { Some(folder) } else { None })
    }

    pub fn recursive_posts(&self) -> Vec<&Post> {
        self.entries.iter()
            .flat_map(|entry| {
                match entry {
                    FolderEntry::Folder(folder) => folder.recursive_posts(),
                    FolderEntry::Post(post) => vec![post]
                }
            })
            .collect()
    }

    pub fn element_at_path(&self, path: &Path) -> Option<&FolderEntry> {
        let first_component: String = path.components().next().unwrap().as_os_str().to_string_lossy().parse().unwrap();

        if path.components().count() == 1 {
            self.entries.iter().find(|post| post.name() == first_component)
        } else {
            let folder = self.folders().find(|folder| folder.name == first_component)?;
            folder.element_at_path(
                Path::new(&path.components()
                    .skip(1)
                    .map(|component| component.as_os_str().to_string_lossy().parse().unwrap())
                    .collect::<Vec<String>>()
                    .join("/")
                )
            )
        }
    }

    pub fn list(&self, base_path: &Path) -> Vec<PathBuf> {
        self.entries.iter()
            .flat_map(|entry| {
                match entry {
                    FolderEntry::Folder(folder) => folder.list(&base_path.join(&folder.name)),
                    FolderEntry::Post(post) => vec![base_path.join(&post.name)]
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum FolderEntry {
    Post(Post),
    Folder(Folder)
}

impl FolderEntry {
    pub fn name(&self) -> String {
        match self {
            FolderEntry::Post(post) => post.name.clone(),
            FolderEntry::Folder(folder) => folder.name.clone()
        }
    }
}
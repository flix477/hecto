use crate::core::posts::folder::Folder;
use std::path::Path;
use crate::util::path_to_string;
use crate::core::posts::Post;
use crate::core::posts::folders::{folders, posts};
use serde::Serialize;

#[derive(Serialize)]
pub struct FolderView {
    pub title: String,
    pub folders: Vec<FolderLink>,
    pub posts: Vec<PostLink>
}

impl FolderView {
    pub fn new(folder: &Folder) -> Self {
        let folders = folders(folder.entries.iter())
            .map(|folder| FolderLink::new(folder, Path::new(&folder.name)))
            .collect();
        let posts = posts(folder.entries.iter())
            .map(|post| PostLink::new(&post, Path::new(&post.name)))
            .collect();
        Self {
            title: folder.name.clone(),
            folders,
            posts
        }
    }
}

#[derive(Serialize)]
pub struct FolderLink {
    pub title: String,
    pub link: String,
}

impl FolderLink {
    pub fn new(folder: &Folder, path: &Path) -> Self {
        Self {
            title: folder.name.clone(),
            link: path_to_string(path),
        }
    }
}

#[derive(Serialize)]
pub struct PostLink {
    pub title: String,
    pub image: Option<String>,
    pub link: String,
    pub preview: String,
}

impl PostLink {
    pub fn new(post: &Post, path: &Path) -> Self {
        Self {
            title: post.title(),
            image: post.metadata.image.clone(),
            link: path_to_string(path),
            preview: post.metadata.preview.clone()
        }
    }
}
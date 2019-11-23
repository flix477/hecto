use crate::core::posts::folder::Folder;
use crate::core::posts::folders::{folders, posts};
use crate::core::posts::Post;
use crate::util::path_to_string;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct FolderView {
    pub title: String,
    pub folders: Vec<FolderLink>,
    pub posts: Vec<PostLink>,
}

impl FolderView {
    pub fn new(folder: &Folder) -> Self {
        let folders = folders(folder.entries.iter())
            .map(|folder| FolderLink::new(folder, Path::new(&folder.name)))
            .collect();
        let mut posts: Vec<&Post> = posts(folder.entries.iter()).collect();
        posts.sort_by_key(|post| post.creation_date);
        let posts = posts
            .iter()
            .map(|post| PostLink::new(&post, Path::new(&post.name)))
            .collect();
        Self {
            title: folder.name.clone(),
            folders,
            posts,
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
    pub creation_date: String,
    pub reading_time: usize,
}

impl PostLink {
    pub fn new(post: &Post, path: &Path) -> Self {
        let creation_date: DateTime<Utc> = post.creation_date.into();
        let creation_date = creation_date.format("%A %B %d %Y, %H:%M").to_string();

        Self {
            title: post.title(),
            image: post.metadata.image.clone(),
            link: path_to_string(path),
            preview: post.metadata.preview.clone(),
            creation_date,
            reading_time: post.metadata.reading_time,
        }
    }
}

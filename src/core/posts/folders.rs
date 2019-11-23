use crate::core::posts::folder::Folder;
use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::Post;
use crate::util::component_as_string;
use std::path::{Path, PathBuf};

type Entry = FolderEntry<Post, Folder>;

/// Filters a list of folder entries to only get the posts
pub fn posts<'a, T>(entries: T) -> impl Iterator<Item = &'a Post>
where
    T: Iterator<Item = &'a Entry>,
{
    entries.filter_map(|entry| {
        if let FolderEntry::Post(post) = entry {
            Some(post)
        } else {
            None
        }
    })
}

/// Filters a list of folder entries to only get the posts as mutable references
pub fn mut_posts<'a, T>(entries: T) -> impl Iterator<Item = &'a mut Post>
where
    T: Iterator<Item = &'a mut Entry>,
{
    entries.filter_map(|entry| {
        if let FolderEntry::Post(post) = entry {
            Some(post)
        } else {
            None
        }
    })
}

/// Recursively gets all the posts in a folder
pub fn recursive_posts(folder: &Folder) -> Vec<&Post> {
    folder
        .entries
        .iter()
        .flat_map(|entry| match entry {
            FolderEntry::Folder(folder) => recursive_posts(folder),
            FolderEntry::Post(post) => vec![post],
        })
        .collect()
}

/// Filters a list of folder entries to only get the folders
pub fn folders<'a, T>(entries: T) -> impl Iterator<Item = &'a Folder>
where
    T: Iterator<Item = &'a Entry>,
{
    entries.filter_map(|entry| {
        if let FolderEntry::Folder(folder) = entry {
            Some(folder)
        } else {
            None
        }
    })
}

/// Filters a list of folder entries to only get the folders as mutable references
pub fn mut_folders<'a, T>(entries: T) -> impl Iterator<Item = &'a mut Folder>
where
    T: Iterator<Item = &'a mut Entry>,
{
    entries.filter_map(|entry| {
        if let FolderEntry::Folder(folder) = entry {
            Some(folder)
        } else {
            None
        }
    })
}

/// Gets the folder entry at a given path
pub fn element_at<'a>(
    folder: &'a Folder,
    path: &Path
) -> Option<FolderEntry<&'a Post, &'a Folder>> {
    let path = path.strip_prefix("/").unwrap_or(path);

    path.components()
        .map(component_as_string)
        .try_fold(folder.into(), element_at_string)
}

fn element_at_string<'a>(acc: FolderEntry<&'a Post, &'a Folder>, component: String)
               -> Option<FolderEntry<&'a Post, &'a Folder>>
{
    acc.folder()
        .and_then(|folder| folder.get_entry(&component))
        .map(FolderEntry::as_ref)
}

/// Gets the folder entry at a given path as a mutable reference
pub fn mut_element_at<'a>(
    folder: &'a mut Folder,
    path: &Path
) -> Option<FolderEntry<&'a mut Post, &'a mut Folder>> {
    path.components()
        .map(component_as_string)
        .try_fold(folder.into(), mut_element_at_string)
}

fn mut_element_at_string<'a>(acc: FolderEntry<&'a mut Post, &'a mut Folder>, component: String)
                      -> Option<FolderEntry<&'a mut Post, &'a mut Folder>>
{
    acc.folder()
        .and_then(|folder| folder.get_mut_entry(&component))
        .map(FolderEntry::as_mut_ref)
}

/// Gets a list of all the paths to all the posts
pub fn list(folder: &Folder, base_path: &Path) -> Vec<PathBuf> {
    folder
        .entries
        .iter()
        .flat_map(|entry| match entry {
            FolderEntry::Folder(folder) => list(folder, &base_path.join(&folder.name)),
            FolderEntry::Post(post) => vec![base_path.join(&post.name)],
        })
        .collect()
}

use crate::core::posts::folder::Folder;
use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::Post;
use crate::util::component_as_string;
use std::path::{Path, PathBuf};

type Entry = FolderEntry<Post, Folder>;

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

pub fn element_at<'a>(
    folder: &'a Folder,
    path: &Path
) -> Option<FolderEntry<&'a Post, &'a Folder>> {
    path.components()
        .map(component_as_string)
        .try_fold(folder.into(), |acc: FolderEntry<&'a Post, &'a Folder>, component| {
            acc.folder()
                .and_then(|folder|
                    folder.entries.iter()
                        .find(|entry| entry.name() == component)
                )
                .map(FolderEntry::as_ref)
        })
}

pub fn mut_element_at<'a>(
    folder: &'a mut Folder,
    path: &Path
) -> Option<FolderEntry<&'a mut Post, &'a mut Folder>> {
    path.components()
        .map(component_as_string)
        .try_fold(folder.into(), |acc: FolderEntry<&'a mut Post, &'a mut Folder>, component| {
            acc.folder()
                .and_then(|folder|
                    folder.entries.iter_mut()
                        .find(|entry| entry.name() == component)
                )
                .map(FolderEntry::as_mut_ref)
        })
}

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

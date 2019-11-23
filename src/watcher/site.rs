use crate::core::parser::post::parse_post;
use crate::core::posts::folder::Folder;
use crate::core::posts::folder_entry::FolderEntry;
use crate::core::posts::Post;
use crate::core::Hecto;
use crate::util::{boxed_error, os_str_to_string, relative_path};
use crate::watcher::watcher::FsEvent;
use core::fmt;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// TODO: refactor
pub fn handle_site_event(app: Arc<Mutex<Hecto>>) -> Box<dyn Fn(FsEvent) + Send> {
    Box::new(move |event| {
        let event: Option<SiteEvent> = event.into();
        if let Some(event) = event {
            let mut app = app.lock().unwrap();
            match event {
                SiteEvent::NewPost(path) => {
                    std::fs::metadata(&path)
                        .map_err(boxed_error)
                        .and_then(|metadata| parse_post(&path, &metadata))
                        .and_then(|post| {
                            let path = relative_path(&path, &app.config.site_root);
                            add_new_post(&mut app, post, &path).map_err(boxed_error)
                        })
                        .unwrap_or_else(|error| println!("Error adding post: {:?}", error));
                }
                SiteEvent::ModifiedPost(path) => {
                    std::fs::metadata(&path)
                        .map_err(boxed_error)
                        .and_then(|metadata| parse_post(&path, &metadata))
                        .and_then(|post| {
                            let path = relative_path(&path, &app.config.site_root);
                            update_post(&mut app, post, &path).map_err(boxed_error)
                        })
                        .unwrap_or_else(|error| println!("Error updating post: {:?}", error));
                }
                SiteEvent::DeletedPost(path) => {
                    let path = relative_path(&path, &app.config.site_root);
                    remove_post(&mut app.root, &path)
                        .unwrap_or_else(|error| println!("Error removing post: {:?}", error));
                }
                _ => {
                    dbg!(event);
                }
            }
        }
    })
}

#[derive(Debug)]
enum NewPostError {
    ParentNotFound,
}

impl Display for NewPostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find parent folder for new post.")
    }
}

impl Error for NewPostError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn add_new_post(app: &mut Hecto, post: Post, path: &Path) -> Result<(), NewPostError> {
    let parent = get_parent_folder(&mut app.root, &path).ok_or(NewPostError::ParentNotFound)?;
    parent.entries.push(FolderEntry::Post(post));
    Ok(())
}

fn update_post(app: &mut Hecto, modified_post: Post, path: &Path) -> Result<(), NewPostError> {
    let parent = get_parent_folder(&mut app.root, &path).ok_or(NewPostError::ParentNotFound)?;
    let post = parent
        .entries
        .iter_mut()
        .find(|entry| entry.name() == modified_post.name)
        .and_then(|entry| entry.as_mut_ref().post())
        .ok_or(NewPostError::ParentNotFound)?;
    std::mem::replace(post, modified_post);
    Ok(())
}

fn remove_post(root: &mut Folder, path: &Path) -> Result<(), NewPostError> {
    let file_name = os_str_to_string(path.file_name().ok_or(NewPostError::ParentNotFound)?);
    let parent = get_parent_folder(root, &path).ok_or(NewPostError::ParentNotFound)?;
    let index = parent
        .entries
        .iter()
        .position(|entry| {
            if let FolderEntry::Post(post) = entry {
                post.name == file_name
            } else {
                false
            }
        })
        .ok_or(NewPostError::ParentNotFound)?;

    parent.entries.remove(index);

    Ok(())
}

fn get_parent_folder<'a>(root: &'a mut Folder, path: &Path) -> Option<&'a mut Folder> {
    path.parent()
        .and_then(move |parent| root.mut_folder_at_path(parent))
}

#[derive(Debug, Clone)]
enum SiteEvent {
    NewPost(PathBuf),
    ModifiedPost(PathBuf),
    NewFolder(PathBuf),
    DeletedPost(PathBuf),
    DeletedFolder(PathBuf),
}

impl Into<Option<SiteEvent>> for FsEvent {
    fn into(self) -> Option<SiteEvent> {
        match self {
            FsEvent::NewFile(path) => Some(SiteEvent::NewPost(path)),
            FsEvent::ModifiedFile(path) => Some(SiteEvent::ModifiedPost(path)),
            FsEvent::DeletedPath(path) => Some(SiteEvent::DeletedPost(path)),
            _ => None,
        }
    }
}

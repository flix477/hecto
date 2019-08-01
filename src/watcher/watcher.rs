use crossbeam_channel::unbounded;
use notify::event::{ModifyKind, RenameMode};
use notify::{watcher, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::thread::spawn;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum FsEvent {
    NewFile(PathBuf),
    NewFolder(PathBuf),
    ModifiedFile(PathBuf),
    DeletedPath(PathBuf),
}

pub fn initialize_watcher<F: Fn(FsEvent) + Send + 'static>(
    path: &Path,
    extension: &str,
    dispatcher: F,
) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(path);
    let extension = String::from(extension);
    spawn(move || {
        let (tx, rx) = unbounded();
        let mut watcher: RecommendedWatcher = watcher(tx, Duration::from_secs(2)).unwrap();
        watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
                Ok(Ok(event)) => match event.kind {
                    EventKind::Create(_) => {
                        if let Some(path) = event.paths.get(0) {
                            if is_watched_file(path, &extension) {
                                (dispatcher)(FsEvent::NewFile(path.clone()))
                            }
                        }
                    }
                    EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                        if let Some(new_path) = event.paths.get(1) {
                            if is_watched_file(new_path, &extension) {
                                (dispatcher)(FsEvent::NewFile(new_path.clone()))
                            } else if is_watched_folder(new_path) {
                                (dispatcher)(FsEvent::NewFolder(new_path.clone()))
                            }
                        }
                    }
                    EventKind::Modify(_) => {
                        if let Some(path) = event.paths.get(0) {
                            if is_watched_file(path, &extension) {
                                (dispatcher)(FsEvent::ModifiedFile(path.clone()))
                            }
                        }
                    }
                    EventKind::Remove(_) => {
                        if let Some(path) = event.paths.get(0) {
                            if is_watched_path(path, &extension) {
                                (dispatcher)(FsEvent::DeletedPath(path.clone()))
                            }
                        }
                    }
                    _ => {}
                },
                Err(error) => {
                    dbg!(error.description());
                }
                _ => {}
            }
        }
    });

    Ok(())
}

fn is_watched_path(path: &PathBuf, extension: &str) -> bool {
    is_watched_file(path, extension) || is_watched_folder(path)
}

fn is_watched_file(path: &PathBuf, extension: &str) -> bool {
    (if let Some(file_extension) = path.extension() {
        extension == file_extension
    } else {
        false
    }) && !is_hidden_path(path)
}

fn is_watched_folder(path: &PathBuf) -> bool {
    path.extension().is_none() && !is_hidden_path(path)
}

fn is_hidden_path(path: &PathBuf) -> bool {
    path.components()
        .any(|component| component.as_os_str().to_string_lossy().starts_with("."))
}

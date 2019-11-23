use crate::core::Hecto;
use crate::watcher::watcher::FsEvent;
use std::sync::{Arc, Mutex};

pub fn handle_theme_event(app: Arc<Mutex<Hecto>>) -> Box<dyn Fn(FsEvent) + Send> {
    Box::new(move |event| {
        let event: Option<ThemeModifiedEvent> = event.into();
        if event.is_some() {
            let mut app = app.lock().unwrap();
            app.update_theme();
            println!("Theme updated.");
        }
    })
}

#[derive(Debug, Copy, Clone)]
enum ThemeModifiedEvent {
    Page,
    Post,
    Folder,
}

impl Into<Option<ThemeModifiedEvent>> for FsEvent {
    fn into(self) -> Option<ThemeModifiedEvent> {
        match self {
            FsEvent::NewFile(path) | FsEvent::ModifiedFile(path) => {
                if let Some(name) = path.file_stem() {
                    let name = name.to_str().unwrap();
                    match name {
                        "page" => Some(ThemeModifiedEvent::Page),
                        "post" => Some(ThemeModifiedEvent::Post),
                        "folder" => Some(ThemeModifiedEvent::Folder),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

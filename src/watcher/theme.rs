use crate::core::Hecto;
use crate::watcher::watcher::FsEvent;
use std::sync::{Arc, Mutex};

pub fn handle_theme_event(app: Arc<Mutex<Hecto>>) -> Box<dyn Fn(FsEvent) + Send> {
    Box::new(move |event| {
        let event: Option<ThemeModifiedEvent> = event.into();
        if let Some(event) = event {
            let mut app = app.lock().unwrap();
            dbg!(&event);
            app.update_theme();
            match event {
                ThemeModifiedEvent::Page => {
                    // rerender every page
                    app.rerender();
                }
                ThemeModifiedEvent::Post => {
                    // rerender every post
                    app.rerender_posts();
                }
            }
        }
    })
}

#[derive(Debug, Copy, Clone)]
enum ThemeModifiedEvent {
    Page,
    Post,
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

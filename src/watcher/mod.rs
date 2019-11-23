use crate::core::Hecto;
use crate::watcher::site::handle_site_event;
use crate::watcher::theme::handle_theme_event;
use crate::watcher::watcher::{initialize_watcher, FsEvent};
use std::error::Error;
use std::sync::{Arc, Mutex};

mod site;
mod theme;
mod watcher;

#[derive(Debug, Clone)]
pub enum SiteEvent {
    Theme(FsEvent),
    Site(FsEvent),
}

pub fn initialize(app: Arc<Mutex<Hecto>>) -> Result<(), Box<dyn Error>> {
    let state = app.lock().unwrap();
    initialize_watcher(
        &state.config.site_root,
        "md",
        handle_site_event(app.clone()),
    )?;
    initialize_watcher(
        &state.config.theme_path,
        "hbs",
        handle_theme_event(app.clone()),
    )
}

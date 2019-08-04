use std::sync::{Arc, Mutex};
use crate::server::response::{Response, HttpCode};
use crate::server::{MiddlewareResult, Request};
use crate::core::Hecto;
use crate::core::folder::{FolderEntry, Folder};
use crate::core::post::Post;

pub fn stuff<'a>(request: &Request, app: &Arc<Mutex<Hecto>>) -> MiddlewareResult<Arc<Mutex<Hecto>>> {
    let path = request.path;
    let app = app.lock().unwrap();
    let element = app.element_at_path(path);

    let response = if let Some(entry) = element {
        Response::ok(entry.to_html())
    } else {
        Response {
            code: HttpCode::NotFound,
            contents: String::new(),
        }
    };
    MiddlewareResult::End(response)
}

trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for FolderEntry<&Post, &Folder> {
    fn to_html(&self) -> String {
        match self {
            FolderEntry::Post(post) => post.contents.clone(),
            FolderEntry::Folder(_) => String::new(),
        }
    }
}
use std::sync::{Arc, Mutex};
use crate::core::Hecto;
use crate::server::response::{HttpCode, Response};
use crate::server::{MiddlewareResult, Request};
use crate::renderer::ToHtml;

pub fn stuff<'a>(
    request: &Request,
    app: &Arc<Mutex<Hecto>>,
) -> MiddlewareResult<Arc<Mutex<Hecto>>> {
    let path = request.path;
    let app = app.lock().unwrap();
    let element = app.element_at_path(path);

    let response = if let Some(entry) = element {
        entry.to_html(&app.renderer)
            .map(Response::ok)
            .unwrap_or_else(|error| {
                dbg!(error);
                Response::internal_error()
            })
    } else {
        Response {
            code: HttpCode::NotFound,
            contents: String::new(),
        }
    };
    MiddlewareResult::End(response)
}

use crate::core::Hecto;
use crate::renderer::ToHtml;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server, StatusCode};
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn run(app: Arc<Mutex<Hecto>>) {
    let address = app.lock().unwrap().config.address.clone();
    let service = move || service_fn_ok(fetch_page(app.clone()));

    let server = Server::bind(&address)
        .serve(service)
        .map_err(|e| eprintln!("Server error: {}", e));

    println!("Running server at http://{}", address);
    hyper::rt::run(server);
}

pub fn fetch_page(app: Arc<Mutex<Hecto>>) -> impl Fn(Request<Body>) -> Response<Body> {
    move |request| {
        let path = request.uri().path();
        let app = app.lock().unwrap();

        let element = app.element_at_path(Path::new(path));

        if let Some(entry) = element {
            entry
                .to_html(&app.renderer)
                .map(on_render_success)
                .unwrap_or_else(on_render_error)
        } else {
            not_found()
        }
    }
}

fn on_render_success(html: String) -> Response<Body> {
    Response::new(Body::from(html))
}

fn on_render_error(error: Box<dyn Error>) -> Response<Body> {
    eprintln!("Rendering error: {}", error);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

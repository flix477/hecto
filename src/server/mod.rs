use crate::core::Hecto;
use std::sync::{Arc, Mutex};
use hyper::{Server, Response, Body, Request, StatusCode};
use hyper::service::service_fn_ok;
use hyper::rt::Future;
use std::path::Path;
use crate::renderer::ToHtml;

pub fn run(app: Arc<Mutex<Hecto>>) {
    let address = app.lock().unwrap().config.address.clone();
    let service = move || {
        service_fn_ok(fetch_page(app.clone()))
    };

    let server = Server::bind(&address)
        .serve(service)
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}

pub fn fetch_page(app: Arc<Mutex<Hecto>>) -> impl Fn(Request<Body>) -> Response<Body> {
    move |request| {
        let path = request.uri().path();
        let app = app.lock().unwrap();

        let element = app.element_at_path(Path::new(path));
        if let Some(entry) = element {
            entry.to_html(&app.renderer)
                .map(|html| Response::new(Body::from(html)))
                .unwrap_or_else(|error| {
                    eprintln!("{}", error);
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap()
                })
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        }
    }
}
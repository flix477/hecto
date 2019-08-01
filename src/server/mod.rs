use crate::core::folder::{Folder, FolderEntry};
use crate::core::post::Post;
use crate::core::Hecto;
use regex::Regex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Server<T> {
    middlewares: Vec<fn(&Request, &T) -> MiddlewareResult<T>>,
}

impl<T> Server<T> {
    fn handle_connection(&self, mut stream: TcpStream, state: &T) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let value = String::from_utf8_lossy(&buffer);

        let response = if let Some(url) = get_url(&value) {
            let request = Request { path: url };
            self.run_middlewares(&request, state)
        } else {
            Response::bad_request()
        };

        stream.write(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn run_middlewares(&self, request: &Request, state: &T) -> Response {
        for middleware in &self.middlewares {
            let result = (middleware)(request, state);
            if let MiddlewareResult::End(response) = result {
                return response;
            }
        }

        panic!("Invalid middlewares")
    }
}

impl Default for Server<Hecto> {
    fn default() -> Self {
        Self {
            middlewares: vec![stuff],
        }
    }
}

impl Server<Hecto> {
    pub fn run(&self, app: Arc<Mutex<Hecto>>) {
        let listener = {
            let app = app.lock().unwrap();
            TcpListener::bind(app.config.address()).expect(
                format!("Error starting server at address: {}", app.config.address()).as_str(),
            )
        };

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let app = app.lock().unwrap();
            self.handle_connection(stream, &app);
        }
    }
}

pub enum MiddlewareResult<T> {
    Next(T),
    End(Response),
}

pub struct Request<'a> {
    pub path: &'a Path,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub code: HttpCode,
    pub contents: String,
}

impl Response {
    pub fn ok(contents: String) -> Response {
        Response {
            code: HttpCode::OK,
            contents,
        }
    }

    pub fn bad_request() -> Response {
        Response {
            code: HttpCode::OK,
            contents: String::new(),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.code as usize,
            self.code.to_string(),
            self.contents
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HttpCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl ToString for HttpCode {
    fn to_string(&self) -> String {
        let string = match self {
            HttpCode::OK => "OK",
            HttpCode::BadRequest => "Bad Request",
            HttpCode::NotFound => "Not Found",
        };
        string.into()
    }
}

fn stuff<'a>(request: &Request, app: &Hecto) -> MiddlewareResult<Hecto> {
    let path = request.path;
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

pub trait ToHtml {
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

lazy_static! {
    static ref REQUEST_REGEX: Regex = Regex::new(r"^GET /?([^\s]*)/? HTTP/1\.1\r\n").unwrap();
}

fn get_url(value: &str) -> Option<&Path> {
    let captures = REQUEST_REGEX.captures(value)?;
    let capture = captures.get(1)?;
    Some(Path::new(capture.as_str()))
}

#[cfg(test)]
mod tests {
    use crate::server::get_url;

    #[test]
    fn url_parses_correctly() {
        assert!(get_url("GET /test HTTP/1.1\r\n").is_some())
    }
}

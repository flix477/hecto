use crate::core::Hecto;
use crate::server::query::stuff;
use crate::server::response::Response;
use crate::server::thread_pool::ThreadPool;
use crate::util::boxed_error;
use core::fmt;
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

mod query;
mod response;
mod thread_pool;

type Middleware<T> = fn(&Request, &T) -> MiddlewareResult<T>;

pub struct Server {
    thread_pool: ThreadPool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            thread_pool: ThreadPool::new(4),
        }
    }
}

impl Server {
    pub fn run(&self, app: Arc<Mutex<Hecto>>) {
        let listener = {
            let app = app.lock().unwrap();
            TcpListener::bind(app.config.address()).expect(
                format!("Error starting server at address: {}", app.config.address()).as_str(),
            )
        };
        let middlewares: Vec<Middleware<Arc<Mutex<Hecto>>>> = vec![stuff];
        let middlewares = Arc::new(middlewares);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let app = app.clone();
            let middlewares = middlewares.clone();
            self.thread_pool.execute(move || {
                handle_connection(stream, &app, &middlewares).unwrap_or_else(|error| {
                    println!("Error processing connection: {:?}", error);
                });
            });
        }
    }
}

fn handle_connection<T>(
    mut stream: TcpStream,
    state: &T,
    middlewares: &[Middleware<T>],
) -> Result<(), Box<dyn Error>> {
    let response = get_response(&mut stream, state, middlewares);
    stream.write(response.to_string().as_bytes())?;
    stream.flush().map_err(boxed_error)
}

fn get_response<T>(stream: &mut TcpStream, state: &T, middlewares: &[Middleware<T>]) -> Response {
    get_buffer(stream)
        .map(|buffer| String::from_utf8_lossy(&buffer).into_owned())
        .and_then(|buffer| {
            get_url(&buffer)
                .ok_or(RequestError::InvalidUrl)
                .map_err(boxed_error)
        })
        .map(|url| {
            let request = Request { path: &url };
            run_middlewares(&request, state, middlewares)
        })
        .unwrap_or_else(|error| Response::bad_request(error.to_string()))
}

fn run_middlewares<T>(request: &Request, state: &T, middlewares: &[Middleware<T>]) -> Response {
    for middleware in middlewares {
        let result = (middleware)(request, state);
        if let MiddlewareResult::End(response) = result {
            return response;
        }
    }

    panic!("Invalid middlewares")
}

pub enum MiddlewareResult<T> {
    Next(T),
    End(Response),
}

pub struct Request<'a> {
    pub path: &'a Path,
}

lazy_static! {
    static ref REQUEST_REGEX: Regex = Regex::new(r"^GET /?([^\s]*)/? HTTP/1\.1\r\n").unwrap();
}

fn get_url(value: &str) -> Option<PathBuf> {
    let captures = REQUEST_REGEX.captures(value)?;
    let capture = captures.get(1)?;
    Some(Path::new(capture.as_str()).into())
}

#[derive(Debug)]
enum RequestError {
    InvalidUrl,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid URL used.")
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn get_buffer<'a>(stream: &mut TcpStream) -> Result<[u8; 512], Box<dyn Error>> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use crate::server::get_url;

    #[test]
    fn url_parses_correctly() {
        assert!(get_url("GET /test HTTP/1.1\r\n").is_some())
    }
}

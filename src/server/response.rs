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

    pub fn bad_request(contents: String) -> Response {
        Response {
            code: HttpCode::BadRequest,
            contents,
        }
    }

    pub fn internal_error() -> Response {
        Response {
            code: HttpCode::InternalServerError,
            contents: String::new(),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            self.code as usize,
            self.code.to_string(),
            self.contents.as_bytes().len(),
            self.contents
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HttpCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl ToString for HttpCode {
    fn to_string(&self) -> String {
        let string = match self {
            HttpCode::OK => "OK",
            HttpCode::BadRequest => "Bad Request",
            HttpCode::NotFound => "Not Found",
            HttpCode::InternalServerError => "Internal Server Error",
        };
        string.into()
    }
}

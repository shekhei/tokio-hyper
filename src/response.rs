use hyper::{StatusCode, Headers, HttpVersion};

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: Headers,
    version: HttpVersion,
}

impl Response {
    pub fn status(status: StatusCode) -> Response {
        Response {
            status: status,
            .. Response::default()
        }
    }

    pub fn ok() -> Response {
        Response::status(StatusCode::Ok)
    }

    pub fn no_content() -> Response {
        Response::status(StatusCode::NoContent)
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn deconstruct(self) -> (StatusCode, Headers, HttpVersion) {
        (self.status, self.headers, self.version)
    }
}

impl Default for Response {
    fn default() -> Response {
        Response {
            status: StatusCode::Ok,
            headers: Headers::new(),
            version: HttpVersion::Http11,
        }
    }
}

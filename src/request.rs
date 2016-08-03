use hyper::{Method, RequestUri, HttpVersion, Headers};
use hyper::server::{Request as HyperRequest};

#[derive(Debug)]
pub struct Request {
    method: Method,
    uri: RequestUri,
    version: HttpVersion,
    headers: Headers,
}

impl Request {
    /// The `Method`, such as `Get`, `Post`, etc.
    #[inline]
    pub fn method(&self) -> &Method { &self.method }

    /// The headers of the incoming request.
    #[inline]
    pub fn headers(&self) -> &Headers { &self.headers }

    /// The target request-uri for this request.
    #[inline]
    pub fn uri(&self) -> &RequestUri { &self.uri }

    /// The version of HTTP for this request.
    #[inline]
    pub fn version(&self) -> &HttpVersion { &self.version }
}

impl<'a, T> From<HyperRequest<'a, T>> for Request {
    fn from(src: HyperRequest<'a, T>) -> Request {
        let (method, uri, version, headers) = src.deconstruct();

        Request {
            method: method,
            uri: uri,
            version: version,
            headers: headers,
        }
    }
}

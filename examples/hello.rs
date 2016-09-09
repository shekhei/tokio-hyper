extern crate tokio_service;
extern crate futures;

#[macro_use]
extern crate hyper;
extern crate tokio_hyper as http;

use tokio_service::Service;
use futures::{Async, Future, finished, BoxFuture};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct MyService;

impl Service for MyService {
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("REQUEST: {:?}", req);

        // Create the HTTP response with the body
        let resp = http::Message::new(http::Response::ok())
            .with_body(b"this is my message\n".to_vec());

        // Return the response as an immediate future
        finished(resp).boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

pub fn main() {
    http::Server::new()
        .serve(|| MyService)
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}

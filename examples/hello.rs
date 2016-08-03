extern crate tokio;
extern crate futures;

#[macro_use]
extern crate hyper;
extern crate tokio_hyper as http;

use tokio::Service;
use futures::{Future, finished};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct MyService;

impl Service for MyService {
    type Req = http::Message<http::Request>;
    type Resp = http::Message<http::Response>;
    type Error = http::Error;
    type Fut = Box<Future<Item = Self::Resp, Error = http::Error>>;

    fn call(&self, req: Self::Req) -> Self::Fut {
        println!("REQUEST: {:?}", req);

        // Create the HTTP response with the body
        let resp = http::Message::new(http::Response::ok())
            .with_body(b"this is my message\n".to_vec());

        // Return the response as an immediate future
        finished(resp).boxed()
    }
}

pub fn main() {
    http::Server::new()
        .serve(MyService)
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}

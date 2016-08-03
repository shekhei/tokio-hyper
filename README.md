# Hyper / Tokio integration

Provides a Tokio Service API on top of Hyper.

## Status

This is currently a preview release and is **not** production ready.
Work to integrate Hyper more deeply with Tokio is on going and will
hopefully land in Hyper 0.10.

## Usage

Here is how you use the Hyper Service API:

```rust
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
```

For more details on the `Service` trait, see the Tokio documentation.

## License

Tokio is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.

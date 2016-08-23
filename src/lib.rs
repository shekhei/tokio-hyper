extern crate tokio;
extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;

mod message;
mod request;
mod response;

pub use message::Message;
pub use request::Request;
pub use response::Response;
pub use hyper::Error;
pub use hyper::server::{Request as HyperRequest, Response as HyperResponse};

use tokio::{NewService, Service};
use hyper::{Control, Decoder, Encoder, HttpStream, Next};
use hyper::server::{Server as HyperServer, Handler};
use futures::{Future, Task};
use std::net::SocketAddr;
use std::{io, thread};
use std::sync::{Arc, Mutex};

pub struct Server {
    addr: Option<SocketAddr>,
}

/// Handles hyper requests
struct ServerHandler<T> {
    // TODO: Improve the implementation
    control: Option<Control>,
    req_head: Option<Request>,
    req_body: Option<Vec<u8>>,
    response: Arc<Mutex<Option<Result<Message<Response>, Error>>>>,
    response_body: Vec<u8>,
    response_cursor: usize,
    service: T,
}

impl Server {
    pub fn new() -> Server {
        Server {
            addr: None,
        }
    }

    pub fn bind(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn serve<T>(self, new_service: T) -> io::Result<()>
        where T: NewService<Req = Message<Request>, Resp = Message<Response>, Error = Error> + Send + 'static,
    {
        let addr = self.addr.unwrap_or_else(|| "0.0.0.0:12345".parse().unwrap());

        let (_handle, server) = HyperServer::http(&addr).unwrap()
            .handle(move |ctrl| {
                ServerHandler::new(
                    ctrl,
                    new_service.new_service().unwrap())
            }).unwrap();

        thread::spawn(move || {
            server.run();
        });

        Ok(())
    }
}

impl<T> ServerHandler<T> {
    fn new(ctrl: Control, service: T) -> ServerHandler<T> {
        ServerHandler {
            control: Some(ctrl),
            req_head: None,
            req_body: Some(vec![]),
            response: Arc::new(Mutex::new(None)),
            response_body: vec![],
            response_cursor: 0,
            service: service,
        }
    }
}

impl<T> Handler<HttpStream> for ServerHandler<T>
    where T: Service<Req = Message<Request>, Resp = Message<Response>, Error = Error>
{
    fn on_request(&mut self, req: HyperRequest<HttpStream>) -> Next {
        self.req_head = Some(Request::from(req));
        Next::read()
    }

    fn on_request_readable(&mut self, decoder: &mut Decoder<HttpStream>) -> Next {
        // Make the borrow checker happy
        let mut body = self.req_body.take().unwrap();

        let start_len = body.len();
        let mut len = start_len;

        body.resize(len + 4_096, 0);

        if let Some(n) = decoder.try_read(&mut body[len..]).unwrap() {
            len = start_len + n;

            body.truncate(len);

            if n == 0 {
                let head = self.req_head.take().unwrap();

                let msg = Message::new(head).with_body(body);

                // Invoke the service
                let resp_fut = self.service.call(msg);

                // Process the future
                let ctrl = self.control.take().unwrap();
                let dst = self.response.clone();

                let resp_fut = resp_fut.then(move |res| {
                    *dst.lock().unwrap() = Some(res);
                    ctrl.ready(Next::write()).unwrap();
                    Ok(())
                });

                // Run the future
                Task::new().run(Box::new(resp_fut));

                return Next::wait()
            }
        } else {
            body.truncate(start_len);
        }

        self.req_body = Some(body);

        Next::read()
    }

    fn on_response(&mut self, res: &mut HyperResponse) -> Next {
        let response = self.response.lock().unwrap()
            .take().unwrap();

        let (head, body) = match response {
            Ok(resp) => resp.deconstruct(),
            Err(_) => unimplemented!(),
        };

        self.response_body = body;

        let (status, headers, _) = head.deconstruct();

        res.set_status(status);
        *res.headers_mut() = headers;

        Next::write()
    }

    fn on_response_writable(&mut self, encoder: &mut Encoder<HttpStream>) -> Next {
        if self.response_cursor == self.response_body.len() {
            return Next::end();
        }

        if let Some(n) = encoder.try_write(&self.response_body[self.response_cursor..]).unwrap() {
            self.response_cursor += n;
        }

        Next::write()
    }
}

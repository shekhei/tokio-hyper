
/// An HTTP message composed of an HTTP head and optional body
#[derive(Debug)]
pub struct Message<H> {
    head: H,
    body: Vec<u8>,
}

impl<H> Message<H> {
    /// Create a new Http message with the given head
    pub fn new(head: H) -> Message<H> {
        Message {
            head: head,
            body: vec![],
        }
    }

    /// Returns a ref to the Head
    pub fn head(&self) -> &H {
        &self.head
    }

    /// Returns a ref to the body
    pub fn body(&self) -> &[u8] {
        &self.body[..]
    }

    /// Set the body
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    /// Deconstruct the message into its components
    pub fn deconstruct(self) -> (H, Vec<u8>) {
        (self.head, self.body)
    }
}

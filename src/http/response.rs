use super::StatusCode;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body: String = match &self.body {
            Some(b) => String::from(b),
            None => String::from(""),
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", 
               self.status_code, 
               self.status_code.reason_phrase(), 
               body)
    }
}


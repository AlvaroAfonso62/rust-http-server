use crate::http::{Request, Response, StatusCode, ParseError};
use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::{Read, Write};

const BUFFER_LEN: usize = 1024;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!(
            "Starting http-server and listening in current address: {} ",
            self.addr
        );
        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

       'main: loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!(
                        "Connection established with success, client: {}",
                        addr.to_string()
                    );
                    let mut buffer: [u8; BUFFER_LEN] = [0; BUFFER_LEN];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Message: {}", String::from_utf8_lossy(&buffer));
                            let response: Response = match Request::try_from(&buffer[..]) {
                                Ok(request) =>handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Falied to send response: {}", e);
                            }

                        },
                        Err(error) => {
                            println!("Fail to read connection, Error: {}", error);
                        }
                    
                    };
                }
                Err(error) => {
                    println!("Falied to established connection: {}", error);
                    continue 'main;
                }
            };
        }
    }
}

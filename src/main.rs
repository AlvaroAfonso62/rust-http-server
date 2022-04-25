#![allow(dead_code)]

use std::env;
use server::Server;
use website_handler::WebsiteHandler;

use http::Method;
use http::Request;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("HTTP_PUBLIC_PATH").unwrap_or(default_path);

    println!("Public path is: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

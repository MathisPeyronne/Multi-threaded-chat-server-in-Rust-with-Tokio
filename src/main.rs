#![allow(dead_code)]

use futures::lock::Mutex;
use server::Server;
use std::{env, sync::Arc};
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    //let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let public_path = env::current_dir()
        .expect("failed here 2")
        .into_os_string()
        .into_string()
        .expect("failed here 1");
    println!("public_path: {}", public_path);
    let server = Server::new(
        "0.0.0.0:8080".to_string(),
        Arc::new(Mutex::new(vec![(
            "Welcome to this chat server :)".to_string(),
            "Admin".to_string(),
        )])),
    );
    server.run(WebsiteHandler::new(public_path));
}

#![allow(dead_code)]

use futures::lock::Mutex;
use pnet::datalink;
use server::Server;
use std::{env, sync::Arc};
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    println!("hello2");
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));

    //****************ip address***********************

    // Get a vector with all network interfaces found
    let all_interfaces = datalink::interfaces();

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    match default_interface {
        Some(interface) => println!(
            "Found default interface with [{}] and ipaddr of [{:?}].",
            interface.name, interface.ips[0]
        ),
        None => println!("Error while finding the default interface."),
    }

    for iface in all_interfaces {
        println!("{:?}", iface.ips);
    }
    let ip_address = "192.168.1.152".to_string();

    //************************************************

    //let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // let public_path = env::current_dir()
    //     .expect("failed here 2")
    //     .into_os_string()
    //     .into_string()
    //     .expect("failed here 1");
    let public_path = "/public".to_string();
    println!("public_path: {}", public_path);
    let server = Server::new(
        "0.0.0.0:8080".to_string(),
        Arc::new(Mutex::new(vec![(
            "Welcome to this chat server :)".to_string(),
            "Admin".to_string(),
        )])),
    );
    server.run(WebsiteHandler::new(public_path, ip_address));
}

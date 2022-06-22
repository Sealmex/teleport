use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::env;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
/// This Struct **SHOULD** Contain All Basic Metadata Informations From a File.
struct FileInfo {
    name: String,
    size: u64,
    data: Vec<u8>,
    last_modified: u32,
    last_accessed: u32,
    creation_time: u32,
}
/// Function to start listening for connections, to send files. Takes a filename of type string
pub fn send(filename: &str) {
    special_print("Starting...");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream_listener in listener.incoming() {
        let stream = stream_listener.unwrap();
        println!("Connection Established!");
        special_print(format!("Connected to {:?}", stream.peer_addr().unwrap()).as_str()); // 3 Methods in a row :v
        handle_connections_send(stream, filename.to_owned());
    }
}

/// Function that will start to connect to certain addresses, to receive files.
pub fn receive() {
    println!("Starting...");
    let addr = "127.0.0.1:8080".parse().unwrap();
    let stream_connect = TcpStream::connect_timeout(&addr, Duration::from_secs(10));
    println!("Connected.");
    handle_connection_receive(stream_connect.unwrap())
}

fn handle_connections_send(stream: TcpStream, filename: String) {
    println!("Sending {}", filename)
}

fn handle_connection_receive(stream: TcpStream) {

}

/// Special Function to only print if a env_variable set early in main.rs is set to enabled
pub fn special_print(text: &str) {
    // Check if var "vb" is set to enabled and print the given text
    if env::var("vb").unwrap() == "enabled" {
        println!("{}", text);
    } else {
    } // Do nothing
}

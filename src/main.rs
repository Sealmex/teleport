use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Waiting for connections...");
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut body = Vec::new();
    println!("Connection establised with {}", stream.peer_addr().unwrap());
    stream.read_to_end(&mut body).unwrap();
    println!("Reading message, Client says {}", from_utf8(&body).unwrap());
}

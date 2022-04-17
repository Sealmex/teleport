use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Waiting for connections...");
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        handle_transfer(stream);
    }
}

fn handle_transfer(mut stream: TcpStream) {
    let mut data = Vec::new();
    println!("Connection establised with {}", stream.peer_addr().unwrap());
    match stream.read_to_end(&mut data) {
        Ok(_) => println!("Read data."),
        Err(_) => println!("Failed to read Data.")
    }
    println!("Data is {}", from_utf8(&data).unwrap())
}

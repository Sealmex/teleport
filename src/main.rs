use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream};
use std::thread;
fn main() {
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Waiting for connections...");
    let mut id = 0;
    for stream in listen.incoming() {
        id += 1;
        thread::spawn(move || 
        {
            let stream = stream.unwrap();
            handle_stream(stream, id);
        });
    }
}

fn handle_stream(mut stream: TcpStream, id: u8) {
    println!("Thread ID is {}", id);
    let mut body = Vec::new();
    println!("Connection establised with {}", stream.peer_addr().unwrap());
    stream.read_to_end(&mut body).unwrap();
    println!("Reading message, Client says {}", from_utf8(&body).unwrap());
}

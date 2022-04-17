use std::io::prelude::*;
use std::str::{from_utf8};
use std::net::{TcpListener, TcpStream};

fn main() {
    // listener
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Waiting for connections...");
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        handle_transfer(stream);
    }
}
// handle streams
fn handle_transfer(mut stream: TcpStream) {
    let mut data = Vec::<u8>::new();
    println!("Connection establised with {}", stream.peer_addr().unwrap());
    match stream.read_to_end(&mut data) {
        Ok(_) => println!("Read data."),
        Err(_) => println!("Failed to read Data.")
    }
    println!("Data is {}", from_utf8(&data).unwrap());
    // debugging
    println!("Raw data {:?}",&data);
    let mut head = Vec::new();
    for id in data {
        head.push(id);
        if id == 226 {
            println!("Found it! {}", id);
            let head_length = from_utf8(&head).unwrap_err();
            println!("Data length before 226 is {:?}", head_length.valid_up_to());
            break;
        } else {
            println!("Doesnt seem to exist")
        }
    }
}



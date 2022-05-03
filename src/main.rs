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
    handle_data(data);
}

fn handle_data(mut data_received: Vec::<u8>) {
    let mut data = data_received;
    fn save_data(filename: &str, filedata: Vec::<u8>){

    }
    let index = data.iter().position(|&x|x == 226).unwrap();
    println!("Index is {}", index);
    println!("Byte in index is {}", data[index])
}



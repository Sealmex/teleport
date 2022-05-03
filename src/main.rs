use std::io::prelude::*;
use std::str::{from_utf8};
use std::net::{TcpListener, TcpStream};
use std::fs::write;
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
        Ok(_) => println!("Read head."),
        Err(_) => println!("Failed to read Data.")
    }
    // debugging
    println!("Data is {}", from_utf8(&data).unwrap());
    println!("Raw data {:?}",&data);
    println!("Text is {}", from_utf8(&data).unwrap());
    handle_data(data);
}

fn handle_data(data_received: Vec::<u8>) {
    let data = data_received;
    fn save_data(filename: &str, filedata: Vec::<u8>){
        println!("Head = {}", filename);
        println!("Data = {:?}", filedata);
        match write(filename, filedata) {
            Ok(_) => println!("File saved!"),
            Err(_) => println!("File failed to save!")
        }
    }
    let index = data.iter().position(|&x|x == 0).unwrap();
    
    println!("Index is {}", index);
    println!("Byte in index is {}", data[index]);
    let head = &data[..index];
    // --- find a better way to do this
    let index_b = index + 1;
    let body = &data[index_b..];
    // ---
    println!("{:?}", head);
    println!("{}", from_utf8(head).unwrap());
    println!("{:?}", body);
    println!("{}", from_utf8(body).unwrap());
    save_data(from_utf8(head).unwrap(), body.to_vec())
}



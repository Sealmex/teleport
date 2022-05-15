use std::io::prelude::*;
use std::str::{from_utf8};
use std::net::{TcpListener, TcpStream};
use std::fs::{read, write};
pub fn main() {
    // Main listener
    // Col 31-53 --- Todo: User argument to change what address to listen to
    let listen = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Waiting for connections...");
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}
// Handles new streams
fn handle_stream(mut stream: TcpStream) {
    // Parse and save Data
    fn handle_data(data_received: Vec::<u8>) {
        let data = data_received;
        fn save_data(filename: &str, filedata: Vec::<u8>){
            println!("Head = {}", filename);
            println!("Data = {:?}", filedata);
            // --- Better way to check ?!?
            match read(filename){
                Err(_) => println!("Filename already exist!"),
                Ok(_) => ()
            }
            // --- //
            match write(filename, filedata) {
                Ok(_) => {println!("File saved!");},
                Err(_) => println!("File failed to save!")
            }
        }
        // Checks where's null is located 
        let index = data.iter().position(|&x|x == 0).unwrap();
        // Splice the head and data
        let head = &data[..index];
        // --- Find a better way to do this?
        let index_b = index + 1;
        let body = &data[index_b..];
        // --- //
        save_data(from_utf8(head).unwrap(), body.to_vec())
    }    
    let mut data = Vec::<u8>::new();
    println!("Connection establised with {}", stream.peer_addr().unwrap());
    // Read stream till it end then put the data to a Vector
    match stream.read_to_end(&mut data) {
        Ok(_) => println!("Data Read"),
        Err(_) => println!("Failed to Read Data")
    }
    println!("{:?}", data);
    handle_data(data);
}


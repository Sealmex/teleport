use std::io::prelude::*;
use std::net::TcpStream;
use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    // Simple TCP Client
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        println!("Connected to the server");
        println!("Sending data.");
        // stream.write_all(data).unwrap();
        println!("Sent.")
    } else {
        println!("Couldn't connect to server...");
    }
}
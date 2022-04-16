use std::io::prelude::*;
use std::net::TcpStream;
fn main() {

    // Simple TCP Client
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        let msg = "Hello!";
        println!("Connected to the server!");
        println!("Sending a message.");
        stream.write(msg.as_bytes()).expect("Failed to write message back.");
    } else {
        println!("Couldn't connect to server...");
    }
}
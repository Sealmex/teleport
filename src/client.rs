use std::io::prelude::*;
use std::net::TcpStream;

fn main() {

    // Simple TCP Client
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        let data = "test.png".to_owned() + "⠀" + "pretend this is an image";
        println!("Connected to the server!");
        println!("Sending data.");
        stream.write_all(data.as_bytes());
        println!("Sent.")
    } else {
        println!("Couldn't connect to server...");
    }
}
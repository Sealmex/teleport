use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
fn main() {

    // Simple TCP Client
    // if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
    //     let msg = "Hello!";
    //     println!("Connected to the server!");
    //     println!("Sending a message.");
    //     stream.write(msg.as_bytes()).expect("Failed to write message back.");
    // } else {
    //     println!("Couldn't connect to server...");
    // }

    // Test
    thread::spawn(|| {
        for i in 1..5 {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
                let msg = "Hello!";
                println!("Connected to the server!");
                println!("Sending a message.");
                stream.write(msg.as_bytes()).expect("Failed to write message back.");
            } else {
                println!("Couldn't connect to server...");
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
            let msg = "Im the Client!";
            println!("Connected to the server!");
            println!("Sending a message.");
            stream.write(msg.as_bytes()).expect("Failed to write message back.");
        } else {
            println!("Couldn't connect to server...");
        }
        thread::sleep(Duration::from_millis(1));
    }
}
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
/* Not needed for the current moment
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Data {
    filename: String,
    filesize: u64,
    filedata: Vec<u8>
}
*/
// Function to start listening for connections, to send files.
pub fn send() {
    println!("Starting...");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        println!("Connection Established!");
        handle_connections(stream.unwrap());
    }
    fn handle_connections(stream: TcpStream) {
    }
}
use std::{{self, io::prelude::*}, env, fs::{self, File}, time::SystemTime};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
/// This Struct **SHOULD** Contain All Basic Metadata Informations From a File.
struct FileInfo {
    size: u64,
    data: Vec<u8>,
    last_modified: SystemTime,
    last_accessed: SystemTime,
    creation_time: SystemTime,
}

impl FileInfo {
    fn new(file_path: &str) -> FileInfo {
        FileInfo {
            size: fs::metadata(file_path).unwrap().len(),
            last_modified: fs::metadata(file_path).unwrap().modified().unwrap(),
            last_accessed: fs::metadata(file_path).unwrap().accessed().unwrap(),
            creation_time: fs::metadata(file_path).unwrap().created().unwrap(),
            data: fs::read(file_path).unwrap(),
        }
    }
}
/// Function to start listening for connections, to send files. Takes a filename of type string
pub fn send(file_path: &str) {
    println!("Info: Connecting with 127.0.0.1:8080"); // Temp
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream_listener in listener.incoming() {
        let stream = stream_listener.unwrap();
        println!("Info: Connection Established!");
        special_print(format!("Connected to {:?}", stream.peer_addr().unwrap()).as_str()); // 3 Methods in a row :v
        handle_connections_send(stream, file_path.to_owned());
    }
}

/// Function that will start to connect to certain addresses, to receive files.
pub fn receive(file_path: &str) {
    println!("Info: Connecting to 127.0.0.1:8080"); // Temp
    let addr = "127.0.0.1:8080".parse().unwrap();
    let stream_connect = TcpStream::connect_timeout(&addr, Duration::from_secs(10));
    let stream = stream_connect.unwrap();
    println!("Info: Connected.");
    special_print(format!("Connected to {:?}", stream.peer_addr().unwrap()).as_str());
    handle_connection_receive(stream)
}

fn handle_connections_send(stream: TcpStream, file_path: String) {
    special_print("Initializing data struct.");
    let file: FileInfo = FileInfo::new(&file_path);
    special_print(format!("{:?}", file).as_str());
}

fn handle_connection_receive(stream: TcpStream) {

}

/// Println! macro wrapped with a simple check for a env variable to actually do stuff
pub fn special_print(text: &str) {
    // Check if var "vb" is set to enabled and print the given text
    if env::var("vb").unwrap() == "enabled" {
        println!("Info_Extra: {}", text);
    } else {
    } // Do nothing
}

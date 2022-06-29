use std::{{io::prelude::*}, env, fs, time::SystemTime, net::SocketAddr, str::from_utf8};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use fs_set_times::{set_atime, set_mtime, SystemTimeSpec};
#[derive(Serialize, Deserialize, Debug)]
/// This Struct **SHOULD** Contain All Basic Metadata Informations From a File.
struct FileInfo {
    size: u64,
    data: Vec<u8>,
    last_modified: SystemTime,
    last_accessed: SystemTime,
}

impl FileInfo {
    fn new(file_path: &str) -> FileInfo {
        FileInfo {
            size: fs::metadata(file_path).unwrap().len(),
            last_modified: fs::metadata(file_path).unwrap().modified().unwrap(),
            last_accessed: fs::metadata(file_path).unwrap().accessed().unwrap(),
            data: fs::read(file_path).unwrap(),
        }
    }
}
/// Function to start listening for connections, to send files. Takes a filename of type string
pub fn send(file_path: &str) {
    println!("Info: Connecting to 127.0.0.1:8080"); // Temp
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
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let stream_connect = TcpStream::connect(addr).expect("Error: Failed to Connect to Client!");
    let stream = stream_connect;
    println!("Info: Connected.");
    special_print(format!("Connected to {:?}", stream.peer_addr().unwrap()).as_str());
    handle_connection_receive(stream, file_path)
}

fn handle_connections_send(mut stream: TcpStream, file_path: String) {
    special_print("Initializing data struct.");
    let file = FileInfo::new(&file_path);
    // special_print(format!("{:?}", file).as_str());
    special_print(format!("Sending File to {:?}", stream.peer_addr().unwrap()).as_str());
    let serialized_data = serde_json::to_string(&file).unwrap();
    stream.write_all(serialized_data.as_bytes()).expect("Error: Failed to write data to stream!");
}

fn handle_connection_receive(mut stream: TcpStream,file_path: &str) {
    fn write_to_file(file_path: &str, file_struct: FileInfo) {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(&file_struct.data).unwrap();
        set_atime(file_path, SystemTimeSpec::Absolute(file_struct.last_accessed)).unwrap();
        set_mtime(file_path, SystemTimeSpec::Absolute(file_struct.last_modified)).unwrap();
        println!("Successfully saved file!");
    }
    let mut buffer = String::new();
    let received_data = stream.read_to_string(&mut buffer);
    special_print(format!("Bytes Read: {:?}", received_data.unwrap()).as_str());
    let file: FileInfo = serde_json::from_str(&buffer).unwrap();
    special_print(format!("Data Received: {:?}", from_utf8(&file.data).unwrap()).as_str());
    special_print(format!("Writing file to filepath: {:?}", file_path).as_str());
    write_to_file(file_path, file)
}

/// Println! macro wrapped with a simple check for a env variable to actually do stuff
/// TODO -> Implement format arguments instead of using format! every time!
pub fn special_print(text: &str) {
    // Check if var "vb" is set to enabled and print the given text
    if env::var("vb").unwrap() == "enabled" {
        println!("Info_Extra: {}", text);
    } else {
    } // Do nothing
}

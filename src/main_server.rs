// use std::io::prelude::*;
// use std::net::{TcpListener, TcpStream};
// use std::fs::{read, write};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Data {
    filename: String,
    filesize: u64,
    filedata: Vec<u8>
}

pub fn main() {

    let data = Data {
        filename: "test.txt".to_string(),
        filesize: 1000,
        filedata: vec![1,2,4,5,5]
    };
    println!("{}", serde_json::to_string(&data).unwrap())
    
}
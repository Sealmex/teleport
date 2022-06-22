use std::path::{Path};
use clap::Parser;
use std::env;
/// Simple program to Send and Receive files on a Network
#[derive(Parser)]
#[clap(author,version,about)]
struct Args {
    /// Send or Receive files
    #[clap(short, long)]
    operation: String,
    /// Filename to send/receive
    #[clap(short, long)]
    filename: String,
    /// Extra information
    #[clap(short, long, value_parser)]
    verbose: bool
}
fn main() {
    let arguments = Args::parse();
    if arguments.verbose {env::set_var("vb","enabled")} else {env::set_var("vb","disabled")}
    if arguments.operation == "send" {
        if validate(arguments.filename.as_str()) {special_print("Pass")} else {eprintln!("Error: Did not Pass!")}
    } else if arguments.filename == "receive" {
        teleport::receive()
    } else {
        eprintln!("Error: No Supplied Arguments!")
    }
}
/// ### Validate Filenames
/// - Does file already Exist \
/// - Does text provided actually have an extension
fn validate(filename: &str) -> bool {
    let exist_already: bool = Path::new(filename).exists();
    let extension_exist: bool= Path::new(filename).extension().is_some();
    // Checks the return value
    if extension_exist == true {if exist_already == false {special_print("Passed all Checks!"); true} else {special_print("Failed check 2"); false}} else {special_print("Failed check 1"); false}
}
/// Special Print
/// Only print if Verbose Env Variable is set
pub fn special_print(text: &str) {
    // Check if debug assertion flag is set and print the given text
    if env::var("vb").unwrap() == "enabled" {
        println!("{}", text);
    } else {} // Do nothing 
}
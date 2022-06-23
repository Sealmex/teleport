use clap::Parser;
use std::env;
use std::path::Path;
use teleport::special_print;
/// Simple program to Send and Receive files on a Network
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Send or Receive files
    operation: String,
    /// File to send/Name for the received file
    filename: String,
    /// Extra information
    #[clap(short, long)]
    verbose: bool,
}
fn main() {
    // Parse arguments
    let arguments = Args::parse();
    // Check if verbose flag is set, if set set an env variable called vb to enabled and for the reverse to disabled
    if arguments.verbose {
        env::set_var("vb", "enabled")
    } else {
        env::set_var("vb", "disabled")
    }
    // Find New Solution!
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    let file = current_dir + "/" + arguments.filename.as_str();
    if arguments.operation == "send" {
        validate(file.as_str(), "send");
        teleport::send(file.as_str())
    } else if arguments.operation == "receive" {
        validate(file.as_str(), "receive");
        teleport::receive(file.as_str())
    } else {
        println!("Error: No Supplied Arguments!")
    }
}
/// ### Validate Filenames
/// - Does file already Exist \
/// - Does text provided actually have an extension \
/// 
/// *Case variable used only for the send operation cuz to send a file you **need** a file, so it changes the behavior on exist_already check*
fn validate(file: &str, case: &str) {
    special_print(format!("Path to file is {}", file).as_str());
    let exist_already: bool = Path::new(file).exists();
    let extension_exist: bool = Path::new(file).extension().is_some();
    // New usecase, what if a user want to send a file without an extension?
    match extension_exist {
        true => {}
        false => {panic!("Error: Provided filename doesn't have an extension!")}
    }
    // if it's a type "send" operation it will change the exist_already check to false so it doesn't panic but only if exist_already is set to true
    match case {
        "send" => {if exist_already == false {panic!("Error: {} doesn't exist!",file)}}
        "receive" => {if exist_already == true {panic!("Error: File already exist! Delete file or change the filename")}}
        _ => {} // Should be fine!
    }
}
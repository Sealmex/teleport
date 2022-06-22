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
    /// File to send/receive
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
    if arguments.operation == "send" {
        validate(arguments.filename.as_str(), "send");
        teleport::send(arguments.filename.as_str())
    } else if arguments.operation == "receive" {
        teleport::receive()
    } else {
        eprintln!("Error: No Supplied Arguments!")
    }
}
/// ### Validate Filenames
/// - Does file already Exist \
/// - Does text provided actually have an extension \
/// *Case variable used only for the send operation cuz to send a file you **need** a file, so it changes the behavior on exist_already check*
fn validate(filename: &str, case: &str) -> bool {
    let mut exist_already: bool = Path::new(filename).exists();
    let extension_exist: bool = Path::new(filename).extension().is_some();
    // This if statement checks the operation type, if its "send" then it will change the exist_already value to false to pass the last if statement
    if case == "send" {
        exist_already = false
    };
    // If statement to actually check the 2 variable
    if extension_exist == true {
        if exist_already == false {
            special_print("Passed all Checks!");
            true
        } else {
            special_print("Failed check 2");
            false
        }
    } else {
        special_print("Failed check 1");
        false
    } // This kinda is more confusing?
}
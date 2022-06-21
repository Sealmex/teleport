use std::{env, path::{Path}};
fn main() {
    // Confusing Naming Conventions.
    let arguments = env::args();
    let mut args = Vec::new();
    for argument in arguments {args.push(argument)};
    if args[1] == "send" {
        if validate(args[2].as_str()) {debug_print("Pass")} else {eprintln!("Error: Did not Pass!")}
    } else if args[1] == "receive" {
        teleport::receive()
    } else if args.is_empty() {
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
    if extension_exist == true {if exist_already == false {debug_print("Passed all Checks!"); true} else {debug_print("Failed check 2"); false}} else {debug_print("Failed check 1"); false}
}
/// Debug Print
/// Only print if Debug Variable is set
pub fn debug_print(text: &str) {
    // Check if debug assertion flag is set and print the given text
    if cfg!(debug_assertions) {
        println!("{}", text);
    } else {} // Do nothing
}
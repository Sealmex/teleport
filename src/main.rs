use std::env::{args};
mod main_server;

fn main() {
    let args: Vec<String> = args().collect();
    
    match args.iter().find(|&x| x == "h-") {
        Some::<&String>(_) => {println!("Teleport\n\nA rust based file sharing program"); return},
        _ => {}

    }
    main_server::main();
}
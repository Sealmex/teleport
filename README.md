# Teleport
### A Simple file sharing program written on [**Rust**](https://rust-lang.org), Trying to be as user friendly a cli program can be.

# Features TODO
  - [ ] Finish it // *far from finished*
  - [ ] Listen to different addresses.
  - [x] Verbose output. *additional detail*
  - [ ] Change the filename after transfer finish // *Maybe Implement by changing the filename after saving or maybe send the file without a set name?*

### Dev Logs
  - [June 19, 2022] Major Refactoring Changes. Using **lib.rs** than trying to use **mod** on stuff, **main.rs** is now going to handle user inputs and anything that might be added in the future, still wondering on how to do verbose output though. Planning to use serde to send json data so i dont have to make my own unreliable and shitty parser.
  - [June 21, 2022] Not much today, Finished the Validate function and a new public function called debug_print that only run if its a debug build.
  - [June 22, 2022] Using [Clap](https://crates.io/crates/clap) I was able to make a cli interface for the actual app and Implemented the verbose feature I want using a cheeky litle env variable that's checked every time it ran and moved the function (debug_print previously now special_print) to lib.rs.
  - [June 23, 2022] Because work on the interface is pretty much done, I started working on actually Sending and Receiving files. The FileInfo struct had some upgrades, First of all, The 3 fields related to time data now use SystemTime instead of u32 and Second of all, FileInfo now have a function to initialize a new struct with only the file_path as arguments. Also refactored main.rs a little bit
> This project is supposed to be my way to learn **Rust** and Programming in General. This explains my indecisive behaviour on this project because Im trying to learn how to do stuff.
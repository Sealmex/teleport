# Teleport
### A Simple file sharing program written on [**Rust**](https://rust-lang.org), Trying to be as user friendly a cli program can be.

# TODO
* # Rewrite the entire codebase to use seperate stream instead of one 
* Finish it
  - [ ] Listen to different addresses.
  - [ ] Verbose output. *additional details* // *Maybe implement using macros?*
  - [ ] Change the filename after transfer finish // *Maybe Implement by changing the filename after saving*

### Dev Logs
  - Major Refactoring Changes. Using **lib.rs** than trying to use **mod** on stuff, **main.rs** is now going to handle user inputs and anything that might be added in the future, still wondering on how to do verbose output though. Planning to use serde to send json data so i dont have to make my own unreliable and shitty parser.

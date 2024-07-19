use std::{env,process::{self}};
use grep::{Command, handler};

/// Main function
fn main() {
    // parse args
    let args: Vec<String> = env::args().collect();
    let command = Command::of(&args).unwrap_or_else(|err|{
        eprintln!("Parse argument fail: {err}");
        eprintln!("Usage: {} keyword file", &args[0]);
        process::exit(1);
    });
    // log
    println!("Searching for \x1B[33m'{}'\x1B[0m", command.key_word);
    println!("In file \x1B[32m'{}'\x1B[0m", command.file_path);
    // hanlder
    if let Err(e) = handler(command){
        eprintln!("handler fail: {e}");
        process::exit(1);
    }
}

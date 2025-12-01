use std::os;
use std::thread::current;
use std::{env::current_dir, fs};

pub fn execute() {
    let current_path = current_dir();

    match current_path {
        Ok(path) => println!("{}", path.display()),
        Err(err) => println!("Something went wrong: {}", err),
    }
}

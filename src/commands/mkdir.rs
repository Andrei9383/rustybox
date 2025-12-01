use std::{fs, process::exit};

pub fn execute(args: &[String]) {
    if args.is_empty() {
        println!("Usage: mkdir ,,.");
        exit(-30);
    }

    for arg in args {
        match fs::create_dir(arg) {
            Ok(()) => {}
            Err(e) => println!("Error when creating directory: {}", e),
        };
    }
}

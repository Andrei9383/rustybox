use std::{fs, io, process::exit};

pub fn execute(args: &[String]) -> io::Result<()> {
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

    Ok(())
}

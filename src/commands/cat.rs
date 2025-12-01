use std::fs;
use std::process::exit;

pub fn execute(args: &[String]) {
    if args.is_empty() {
        println!("Usage: cat asfdasdf");
        exit(-1);
    }

    for arg in args {
        let file = fs::read_to_string(arg);
        match file {
            Ok(content) => println!("{}", content),
            Err(e) => {
                println!("Error reading the file: {}", e);
                exit(-1);
            }
        }
    }
}

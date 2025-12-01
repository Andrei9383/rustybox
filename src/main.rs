use std::{env, process::exit};
mod commands;

fn display_help() {
    println!("Usage: rustybox [command] [...arguments]");
    exit(-1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let argc = args.len();

    if argc == 1 {
        display_help();
    }

    let program_name = &args[0];
    println!("Program name: {}", program_name);

    let command = &args[1];

    match command.as_str() {
        "ls" => commands::ls::execute(),
        "pwd" => commands::pwd::execute(),
        "echo" => commands::echo::execute(&args[2..]),
        "cat" => commands::cat::execute(&args[2..]),
        "mkdir" => commands::mkdir::execute(&args[2..]),
        _ => println!("Unknown command: {}", command),
    }
}

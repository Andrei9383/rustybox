use std::io::{self, Error};
pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Ok(());
    }

    if args[0] == "-n" {
        print!("{}", args[1..].join(" "));
        return Ok(());
    }

    println!("{}", args.join(" "));

    Ok(())
}

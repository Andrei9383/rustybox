pub fn execute(args: &[String]) {
    if args.is_empty() {
        return;
    }

    if args[0] == "-n" {
        print!("{}", args[1..].join(" "));
        return;
    }

    println!("{}", args.join(" "));
}

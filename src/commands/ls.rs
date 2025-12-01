use std::{
    env::current_dir,
    fs::{self, read_dir},
    io,
};

pub fn execute() -> io::Result<()> {
    let current_dir = current_dir()?;

    let entries = read_dir(current_dir)?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().display());
    }

    Ok(())
}

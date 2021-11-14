use glob::glob;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, Error, stdin, stdout,};


fn main() -> Result<(), Error> {
    let mut current_dir = String::new();
    stdin().read_line(&mut current_dir).expect("Did not enter a correct string");
    let mut file = OpenOptions::new().append(true).open("files.txt");

    for entry in glob(&current_dir).expect("Failed to read current directory") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}



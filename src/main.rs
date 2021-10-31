use glob::glob;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let current_dir: str = env::current_dir().unwrap();
    let mut file = OpenOptions::new().append(true).open("files.txt");

    for entry in glob(&current_dir).expect("Failed to read current directory") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}

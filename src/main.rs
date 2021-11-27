use glob::glob;
use std::io::{Error, stdin};
//use std::env;
//use std::fs::{File, OpenOptions};
//use std::io::{Write, Error, stdin, stdout,};


fn main() -> Result<(), Error> {
    let mut current_dir_input = String::new();
    stdin().read_line(&mut current_dir_input).expect("Did not enter a correct string");
    let trimmed_dir = &current_dir_input.trim_end().to_string();
    let current_dir_to_loop = format!("{}\\**", &trimmed_dir);
    
    // let path_file = OpenOptions::new().append(true).open("files.txt");

    for entry in glob(&current_dir_to_loop).expect("Failed to read current directory") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}



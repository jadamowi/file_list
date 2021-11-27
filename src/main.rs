use glob::glob;
use std::fs;
use std::io::{Error, stdin};


fn main() -> Result<(), Error> {
    let mut current_dir_input = String::new();
    stdin().read_line(&mut current_dir_input).expect("Did not enter a correct string");
    let trimmed_dir = &current_dir_input.trim_end().to_string();
    let current_dir_to_loop = format!("{}\\**\\*", &trimmed_dir);

    let mut paths:Vec<std::path::PathBuf>  = Vec::new();

    for entry in glob(&current_dir_to_loop).expect("Failed to read current directory") {
        match entry {
            Ok(path) => if check_type(&path) {paths.push(path)},
            Err(e) => println!("{:?}", e),
        }
    }
    
    println!("{:?}", paths);

    Ok(())
}


fn check_type(file_buf:&std::path::PathBuf) -> bool {
    let metadata = fs::metadata(&file_buf).unwrap();
    let file_type = metadata.file_type();
    file_type.is_file()
}


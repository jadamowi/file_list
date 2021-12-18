use glob::glob;
use std::fs;
use std::io::{Error, stdin};

mod create_file;
use create_file::create_xlsx;

mod create_df;
use create_df::create_df;


fn main() -> Result<(), Error> {
    let mut current_dir_input = String::new();
    stdin().read_line(&mut current_dir_input).expect("Did not enter a correct string");
    let trimmed_dir = &current_dir_input.trim_end().to_string();
    let current_dir_to_loop = format!("{}\\**\\*", &trimmed_dir);

    let mut paths:Vec<String>  = Vec::new();

    for entry in glob(&current_dir_to_loop).expect("Failed to read current directory") {
        match entry {
            Ok(path) => if check_type(&path) {
                paths.push(path.into_os_string().into_string().unwrap())
            },
            Err(e) => println!("{:?}", e),
        }
    }
    
    //println!("{:?}", paths);
    //create_xlsx().unwrap();

    let dataf = create_df(paths).unwrap();
    println!("{:?}",dataf);

    Ok(())
}


fn check_type(file_buf:&std::path::PathBuf) -> bool {
    // func checks if an object is a file
    let metadata = fs::metadata(&file_buf).unwrap();
    let file_type = metadata.file_type();
    file_type.is_file()
}





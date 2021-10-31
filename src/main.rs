use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir().unwrap();

    let path = "files.txt";

    let mut output = File::create(path)?;
    write!(output, "{}", current_dir.display())?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

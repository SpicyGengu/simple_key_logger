use std::fs::{self, File};
use std::time::{Duration, Instant};
use std::io::prelude::*;

fn write_to_file(path: &str, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &contents)?;
    Ok(())
}

fn main() {
    let file_name: &str = "logs.txt";
    let mut log_content: &str = "now look at this";
    match write_to_file(&file_name, &log_content) {
        Ok(_) => (),
        Err(_) => println!("couldn't write to file")
    }
}

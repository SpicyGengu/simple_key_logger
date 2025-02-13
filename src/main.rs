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
    let mut log_content_addon: &str = "\nstrong or smtn";
    match write_to_file(&file_name, &log_content) {
        Ok(_) => (),
        Err(_) => println!("couldn't write to file")
    }
    let binding = (log_content.to_owned() + log_content_addon);
    log_content = binding.as_str();
    match write_to_file(&file_name, &log_content) {
        Ok(_) => (),
        Err(_) => println!("couldn't write to file")
    }
}

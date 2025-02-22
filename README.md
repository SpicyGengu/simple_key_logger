# Disclaimer
This project is intended for educational and ethical research purposes only. It is designed to help developers understand how keylogging software works to improve cybersecurity awareness and defensive measures.

Unauthorized use of this software for malicious purposes is strictly prohibited.
Deploying a keylogger without the explicit consent of all parties involved may violate local, national, and international laws. The repository owner, contributors, and maintainers are not responsible for any misuse of this code.

By using or modifying this software, you acknowledge that you are solely responsible for complying with all applicable laws and regulations. If you are unsure about the legal implications, consult a qualified attorney before proceeding.

# About
This keylogger is a little side project I did to familiarize myself with Rust. Essentially, it writes all keys pressed on the "victims" keyboard, writes them down in a txt file, and sends the txt file via email every interval specified. Mind you as of now the exe file compiled by the compiler will mark as a virus.

# Setup
The code works as intended in rustc 1.85.0, all newer and older might work but weren't tested on.
## Pre-requisits
Frirst of you will need to have rust compiler installed to be able to compile the program.

In the case you are using gmail for the project you will need to create an app-password. I'm not sure how it works with other email addresses, but I am certain it's possible.
## Code personalization 
Follow the code comments in main.rs for easier navigation
- Write the receiver email address in the to_mail_address variable
- Write the sender email address in the from_mail_address variable (in my case it's the same address)
- Write the password in the from_mail_address_password (if you are using gmail you need to write the app-password and not your account password)
- Change how often a mail is sent in mail_sending_interval (by default it's set to send a mail every 10 sec)
- Some other variables are also up for editing but are a little more specific (may comments help you if needed)

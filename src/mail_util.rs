use std::fs;
use std::env;

use lettre::{
    Message, SmtpTransport, Transport,
    message::header::ContentType,
    transport::smtp::authentication::{Credentials},
};
use crate::config;

pub fn send_mail(
    config: &config::Config,
    subject: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(config.sender_email.parse()?)
        .to(config.receiver_email.parse()?)
        .subject(subject.to_string())
        .header(ContentType::TEXT_PLAIN)
        .body(content.to_string())?;

    let mail = config.sender_email.to_owned();
    let password = get_credentials();
    let password = match password {
        Ok(password) => password,
        Err(error) => panic!("{}", error),
    };

    let sender = SmtpTransport::starttls_relay("smtp.gmail.com")?
        .credentials(Credentials::new(mail, password))
        .build();

    sender.send(&email)?;
    return Ok(());
}

fn get_credentials() -> Result<String, Box<dyn std::error::Error>> {
    let credentials_dir = env::var("CREDENTIALS_DIRECTORY")?;

    let password_path = format!("{}/smtp-password", credentials_dir);

    let password = fs::read_to_string(password_path);

    return Ok(password?);
}

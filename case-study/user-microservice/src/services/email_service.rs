/*use crate::errors::error::AuthError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
//use std::env;

pub fn send_mail(email_to: String, subject: String, text: String) -> Result<String, AuthError> {
    trace!("Sending email...");
    let email = Message::builder()
        .from("noreply.kts.l9@gmail.com".parse().unwrap())
        .to("jelenacupac99@gmail.com".parse().unwrap())
        .subject(&subject)
        .body(text)
        .unwrap();

    let creds = Credentials::new(
        "noreply.kts.l9@gmail.com".to_string(),
        "ktsrandom".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            info!(
                "{}",
                format!(
                    "Email sent successfully for {} with subject {}",
                    &email_to, &subject
                )
            );
            Ok("Email sent successfully!".to_string())
        }
        Err(e) => {
            debug!("{}", format!("Could not send mail: {}", e.to_string()));
            Err(AuthError::ProcessError(format!(
                "Could not send email: {:?}",
                e
            )))
        }
    }
}*/
/*
fn smtp_username() -> String {
    env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set")
}

fn smtp_password() -> String {
    env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set")
}
*/

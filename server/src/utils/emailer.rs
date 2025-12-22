use serde::{Deserialize, Serialize};
use lettre::message::{header::ContentType, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;

use crate::core::exceptions::auth_exceptions::AuthError;

const SMTP_HOST: &str = "smtp.gmail.com";

#[derive(Serialize, Deserialize)]
pub struct RegisterValidateClaims {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub exp: usize
}

pub async fn registration_verification(email: &str, token: &str) -> Result<(), AuthError> {
    let verification_link = format!("http://localhost:5000/auth/verifyregister?token={}", token);
    
    let body = format!(
        r#"
        <p>Click the link to complete your registration:</p>
        <a href="{link}">{link}</a>
        "#,
        link = verification_link
    );

    send_email(email, "Verify your email", &body).await
}

// pub async fn update_email_verification(new_email: &str, token: &str) -> Result<(), AuthError> {
//     let verification_link = format!("http://localhost:5000/auth/verifyemail?token={token}"); // TODOO need to create handler
//     let body = format!(
//         r#"
//         <p>Click the link to finish updating your email:</p>
//         <a href="{link}">{link}</a>
//         "#,
//         link = verification_link
//     );

//     send_email(new_email, "Verify your email", &body).await
// }

// Send the email with the transport
async fn send_email(recipient: &str, subject: &str, html_body: &str) -> Result<(), AuthError> {
    let email_user = env::var("EMAIL_USER").map_err(|_| AuthError::Email)?;
    let email_pass = env::var("EMAIL_PASS").map_err(|_| AuthError::Email)?;

    let email = build_message(&email_user, recipient, subject, html_body)?;
    let transport = build_transport(&email_user, &email_pass)?;
    transport.send(email).await.map_err(|_| AuthError::Email)?;
    Ok(())
}

// Constructs a lettre::Message
fn build_message(from: &str, to: &str, subject: &str, html_body: &str) -> Result<Message, AuthError> {
    let html_part = SinglePart::builder()
        .header(ContentType::TEXT_HTML)
        .body(html_body.to_string());

    Message::builder()
        .from(parse_mailbox(from)?)
        .to(parse_mailbox(to)?)
        .subject(subject)
        .multipart(MultiPart::alternative().singlepart(html_part))
        .map_err(|_| AuthError::Email)
}

// Create the SMTP client
fn build_transport(user: &str, pass: &str) -> Result<AsyncSmtpTransport<Tokio1Executor>, AuthError> {
    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(SMTP_HOST)
        .map_err(|_| AuthError::Email)?
        .credentials(Credentials::new(user.to_string(), pass.to_string()))
        .build();
    Ok(transport)
}

// Validate email string
fn parse_mailbox(address: &str) -> Result<Mailbox, AuthError> {
    address.parse::<Mailbox>().map_err(|_| AuthError::Email)
}

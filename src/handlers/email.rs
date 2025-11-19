use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use lettre::Transport;
use lettre::{
    Message, SmtpTransport,
    message::{MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use serde_email::Email;

use crate::models::app::AppState;
use crate::models::email::EmailRequest;
use crate::models::email::EmailResponse;

pub async fn send_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> impl IntoResponse {
    // Email validation
    if !payload.email.as_str().contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(EmailResponse {
                success: false,
                message: "Неверный формат email".to_string(),
            }),
        );
    }

    // Email sending
    match send_verification_email(&payload.email, &payload.code, &state).await {
        Ok(_) => (
            StatusCode::OK,
            Json(EmailResponse {
                success: true,
                message: "Письмо успешно отправлено.".to_string(),
            }),
        ),
        Err(e) => {
            eprintln!("Ошибка отправки письма: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(EmailResponse {
                    success: false,
                    message: "Не удалось отправить письмо".to_string(),
                }),
            )
        }
    }
}

// Sending email function
async fn send_verification_email(
    to_email: &Email,
    verification_code: &str,
    state: &AppState,
) -> Result<(), String> {
    println!("🔧 Debug info:");
    println!("  From email: {}", &state.from_email);
    println!("  SMTP username: {}", &state.smtp_username);
    println!("  To email: {}", to_email);
    println!("  Code: {}", verification_code);

    let text_content = format!(
        "Hello!\n\nYout verification code: {}\n\nThe code is valid for 10 minutes. Do not share this code with anyone\n\nIf you did not request this code, please ignore this email.\n",
        verification_code
    );

    let html_content = format!(
        r#"<!DOCTYPE html>
        <html>
            <head><meta charset="UTF-8"></head>
            <body style="font-family: Arial, sans-serif; margin: 20px;">
                <h2>Verification code</h2>
                <p>Your code to complete registration:</p>
                <div style="font-size: 32px; font-weight: bold; letter-spacing: 8px;
                            text-align: center; margin: 25px 0; padding: 15px;
                            background: #f8f9fa; border-radius: 8px; border: 2px dashed #dee2e6;">
                    {}
                </div>
                <p style="color: #6c757d; font-size: 14px;">
                    The code is valid for 10 minutes. Do not share this code with anyone.
                </p>
            </body>
        </html>"#,
        verification_code
    );

    let email = Message::builder()
        .from(
            state
                .from_email
                .parse()
                .map_err(|e| format!("Invalid from email: {}", e))?,
        )
        .to(to_email
            .as_str()
            .parse()
            .map_err(|e| format!("Invalid to email: {}", e))?)
        .subject("Your verification code")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_PLAIN)
                        .body(text_content),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(html_content),
                ),
        )
        .map_err(|e| format!("Failed to build email: {}", e))?;

    let credentials = Credentials::new(state.smtp_username.clone(), state.smtp_password.clone());

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .map_err(|e| format!("Failed to create SMTP relay: {}", e))?
        .credentials(credentials)
        .port(587)
        .build();

    mailer
        .send(&email)
        .map_err(|e| format!("Failed to send email: {}", e))?;

    Ok(())
}

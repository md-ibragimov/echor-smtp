use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Структуры для запроса и ответа
#[derive(Debug, Deserialize)]
struct EmailRequest {
    email: String,
    code: String,
}

#[derive(Debug, Serialize)]
struct EmailResponse {
    success: bool,
    message: String,
}

// Состояние приложения
#[derive(Clone)]
struct AppState {
    smtp_username: String,
    smtp_password: String,
    from_email: String,
}

// Функция для отправки email
async fn send_verification_email(
    to_email: &str,
    verification_code: &str,
    state: &AppState,
) -> Result<(), String> {
    println!("🔧 Debug info:");
    println!("  From email: {}", &state.from_email);
    println!("  SMTP username: {}", &state.smtp_username);
    println!("  To email: {}", to_email);
    println!("  Code: {}", verification_code);
    let email = Message::builder()
        .from(
            state
                .from_email
                .parse()
                .map_err(|e| format!("Invalid from email: {}", e))?,
        )
        .to(to_email
            .parse()
            .map_err(|e| format!("Invalid to email: {}", e))?)
        .subject("Ваш код подтверждения")
        .body(format!(
            "Здравствуйте!\n\nВаш код подтверждения: {}\n\nС уважением,\nКоманда сервиса",
            verification_code
        ))
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

// Статус запуска сервера
async fn health_check() -> &'static str {
    "ok"
}

// Обработчик POST запроса
async fn send_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> impl IntoResponse {
    // Валидация email
    if !payload.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(EmailResponse {
                success: false,
                message: "Неверный формат email".to_string(),
            }),
        );
    }

    // Отправка email
    match send_verification_email(&payload.email, &payload.code, &state).await {
        Ok(_) => (
            StatusCode::OK,
            Json(EmailResponse {
                success: true,
                message: "Письмо успешно отправлено".to_string(),
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

#[tokio::main]
async fn main() {
    // Настройка SMTP credentials
    dotenvy::dotenv().ok();

    let state = AppState {
        smtp_username: std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
        smtp_password: std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
        from_email: std::env::var("FROM_EMAIL").expect("FROM_EMAIL must be set"),
    };

    // Создание маршрута с защитой
    let app = Router::new()
        .route("/", get(health_check))
        .route("/send-email", post(send_email_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
mod handlers;
mod models;
use crate::handlers::email::send_email_handler;
use crate::handlers::health_check::health_check;
use crate::models::app::AppState;

#[tokio::main]
async fn main() {
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
    println!("Server run on this address: http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

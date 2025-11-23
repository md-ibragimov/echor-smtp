use axum::{
    Router,
    routing::{get, post},
};
use axum::middleware as axum_middleware;
use std::net::SocketAddr;
mod handlers;
mod models;
mod middleware;
use crate::handlers::email::send_email_handler;
use crate::handlers::health_check::health_check;
use crate::models::app::AppState;
use serde_email::Email;
use middleware::allowed_origin;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState {
        smtp_username: std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
        smtp_password: std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
        from_email: Email::from_str(std::env::var("FROM_EMAIL").expect("FROM_EMAIL must be set")).expect("Convert str to email error"),
    };

    // Создание маршрута с защитой
    let app = Router::new()
        .route("/", get(health_check))
        .route("/send-email", post(send_email_handler))
        .with_state(state)
        .layer(axum_middleware::from_fn(allowed_origin::ip_based_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server run on this address: http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

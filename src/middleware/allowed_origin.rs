use axum::{
    extract::ConnectInfo,
    http::{StatusCode},
    middleware::{Next},
    response::Response,
};
use std::net::SocketAddr;

pub async fn ip_based_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let allowed_ip = String::from("127.0.0.1");

    let client_ip = addr.ip().to_string();

    // Check ip
    if client_ip != allowed_ip {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

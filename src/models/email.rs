use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct EmailResponse {
    pub success: bool,
    pub message: String,
}

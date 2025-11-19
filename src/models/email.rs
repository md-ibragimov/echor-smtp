use serde::{Deserialize, Serialize};
use serde_email::Email;

#[derive(Debug, Deserialize)]
pub struct EmailRequest {
    pub email: Email,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct EmailResponse {
    pub success: bool,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        let email = Email::from_str("echor.support@gmail.com").unwrap();
        let test_email_request = EmailRequest {
            email,
            code: String::from("4483"),
        };

        assert_eq!(test_email_request.email.as_str(), "echor.support@gmail.com");
    }
}

#[derive(Clone)]
pub struct AppState {
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

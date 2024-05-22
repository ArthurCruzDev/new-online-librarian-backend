use chrono::{DateTime, Utc};
#[derive(Debug, Default)]
pub struct User {
    pub id: Option<u64>,
    pub email: String,
    pub password: String,
    pub email_token: Option<String>,
    pub name: String,
    pub profile_picture: Option<String>,
    pub created_at: DateTime<Utc>,
    pub active: bool,
}

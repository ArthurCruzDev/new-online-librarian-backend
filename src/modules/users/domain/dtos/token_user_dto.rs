use serde::Serialize;

#[derive(Serialize, Default)]
pub struct TokenUserDto {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: Option<String>,
    pub user_name: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
}

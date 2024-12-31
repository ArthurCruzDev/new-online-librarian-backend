use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub profile_picture: Option<String>,
}

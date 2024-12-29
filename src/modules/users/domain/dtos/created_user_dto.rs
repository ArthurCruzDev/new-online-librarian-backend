use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize)]
pub struct CreatedUserDto {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub profile_picture: Option<String>,
}

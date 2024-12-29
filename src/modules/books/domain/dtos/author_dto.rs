use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthorDto {
    pub name: Option<String>,
    pub url: Option<String>,
}

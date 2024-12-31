use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LanguageDto {
    pub name: Option<String>,
    pub code: Option<String>,
}

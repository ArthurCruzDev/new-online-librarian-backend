use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Language {
    pub name: String,
    pub code: Option<String>,
}

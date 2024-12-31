use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

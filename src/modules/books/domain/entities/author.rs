use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Author {
    pub name: String,
    pub url: String,
}

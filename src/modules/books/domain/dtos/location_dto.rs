use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocationDto {
    pub id: Option<u64>,
    pub name: String,
    pub user_id: u64,
}

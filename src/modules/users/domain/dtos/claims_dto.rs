use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ClaimsDto {
    pub id: u64,
    pub exp: i64,
}

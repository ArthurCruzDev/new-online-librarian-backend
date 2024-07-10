use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PaginatedDto<T> {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub items: Vec<T>,
}

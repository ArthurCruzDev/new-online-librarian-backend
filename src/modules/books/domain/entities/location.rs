use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Location {
    pub id: Option<u64>,
    pub name: String,
    pub user_id: u64,
}

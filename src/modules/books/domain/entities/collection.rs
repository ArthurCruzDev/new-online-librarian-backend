use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Collection {
    pub id: Option<u64>,
    pub name: String,
    pub user_id: u64,
}

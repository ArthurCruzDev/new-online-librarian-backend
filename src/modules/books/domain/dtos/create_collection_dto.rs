use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct CreateCollectionDto {
    pub name: Option<String>,
    pub user_id: Option<u64>,
}

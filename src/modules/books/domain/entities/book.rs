use serde::{Deserialize, Serialize};

use super::{author::Author, genre::Genre, language::Language};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<u64>,
    pub title: String,
    pub authors: Vec<Author>,
    pub publisher: String,
    pub languages: Vec<Language>,
    pub edition: Option<String>,
    pub isbn: Option<String>,
    pub year: Option<String>,
    pub genres: Option<Vec<Genre>>,
    pub cover: Option<String>,
    pub collection_id: Option<u64>,
    pub location_id: u64,
    pub user_id: u64,
}

use sqlx::types::Json;

use super::{author::Author, language::Language};

pub struct Book {
    pub id: Option<u64>,
    pub title: String,
    pub authors: Json<Author>,
    pub publisher: String,
    pub languages: Json<Language>,
}

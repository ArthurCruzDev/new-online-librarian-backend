use serde::{Deserialize, Serialize};

use super::{author_dto::AuthorDto, genre_dto::GenreDto, language_dto::LanguageDto};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBookDto {
    pub title: Option<String>,
    pub authors: Option<Vec<AuthorDto>>,
    pub publisher: Option<String>,
    pub languages: Option<Vec<LanguageDto>>,
    pub edition: Option<String>,
    pub isbn: Option<String>,
    pub year: Option<String>,
    pub genres: Option<Vec<GenreDto>>,
    pub cover: Option<String>,
    pub collection_id: Option<u64>,
    pub location_id: Option<u64>,
    pub user_id: Option<u64>,
}

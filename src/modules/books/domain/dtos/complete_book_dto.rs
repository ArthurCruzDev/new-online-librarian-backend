use serde::{Deserialize, Serialize};

use super::{
    author_dto::AuthorDto, collection_dto::CollectionDto, genre_dto::GenreDto,
    language_dto::LanguageDto, location_dto::LocationDto,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompleteBookDto {
    pub id: u64,
    pub title: String,
    pub authors: Vec<AuthorDto>,
    pub publisher: String,
    pub languages: Vec<LanguageDto>,
    pub edition: Option<String>,
    pub isbn: Option<String>,
    pub year: Option<String>,
    pub genres: Option<Vec<GenreDto>>,
    pub cover: Option<String>,
    pub collection: Option<CollectionDto>,
    pub location: LocationDto,
    pub user_id: u64,
}

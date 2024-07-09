use crate::modules::{
    books::domain::{
        dtos::{
            author_dto::AuthorDto, complete_book_dto::CompleteBookDto, genre_dto::GenreDto,
            language_dto::LanguageDto, location_dto::LocationDto,
        },
        entities::book::Book,
    },
    shared::errors::simple_api_error::SimpleAPIError,
};

impl TryFrom<Book> for CompleteBookDto {
    type Error = SimpleAPIError;

    fn try_from(entity: Book) -> Result<Self, Self::Error> {
        let mut dto = CompleteBookDto::default();
        match entity.id {
            Some(entity_id) => dto.id = entity_id,
            None => {
                return Err(SimpleAPIError {
                    msg: "Invalid conversion between entities".to_string(),
                    code: 500,
                });
            }
        }
        dto.title = entity.title;

        for author_entity in entity.authors.into_iter() {
            dto.authors.push(AuthorDto::from(author_entity))
        }

        dto.publisher = entity.publisher;

        for language_entity in entity.languages.into_iter() {
            dto.languages.push(LanguageDto::from(language_entity));
        }

        dto.edition = entity.edition;
        dto.isbn = entity.isbn;
        dto.year = entity.year;
        match entity.genres {
            Some(some_genres) => {
                let mut genre_dto_vec = Vec::with_capacity(some_genres.len());
                for genre_entity in some_genres.into_iter() {
                    genre_dto_vec.push(GenreDto::from(genre_entity))
                }
                dto.genres = Some(genre_dto_vec);
            }
            None => {}
        }
        dto.cover = entity.cover;
        dto.collection = None;
        dto.location = LocationDto::default();
        dto.user_id = entity.user_id;

        Ok(dto)
    }
}

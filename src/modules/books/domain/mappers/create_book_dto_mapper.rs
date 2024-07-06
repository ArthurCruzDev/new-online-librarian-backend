use crate::modules::{
    books::domain::{
        dtos::create_book_dto::CreateBookDto,
        entities::{
            author::{self, Author},
            book::Book,
            genre::Genre,
            language::Language,
        },
    },
    shared::errors::detailed_api_error::DetailedAPIError,
};
use std::collections::HashMap;

impl TryFrom<CreateBookDto> for Book {
    type Error = DetailedAPIError;

    fn try_from(dto: CreateBookDto) -> Result<Self, Self::Error> {
        let mut book = Book::default();
        let mut validations: HashMap<String, String> = HashMap::default();

        match dto.title {
            Some(title) => {
                let candidate_title = title.trim();
                if candidate_title.is_empty() {
                    validations.insert(
                        "title".to_string(),
                        "Book title must not be empty".to_string(),
                    );
                }
                book.title = candidate_title.to_string()
            }
            None => {
                validations.insert(
                    "title".to_string(),
                    "Book title must be informed".to_string(),
                );
            }
        }

        match dto.authors {
            Some(authors) => {
                if authors.is_empty() {
                    validations.insert(
                        "authors".to_string(),
                        "At least one author must be informed".to_string(),
                    );
                } else {
                    let mut authors_entity: Vec<Author> = Vec::with_capacity(authors.capacity());
                    for author_dto in authors.into_iter() {
                        match Author::try_from(author_dto) {
                            Ok(author) => {
                                authors_entity.push(author);
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    book.authors = authors_entity;
                }
            }
            None => {
                validations.insert(
                    "authors".to_string(),
                    "Book authors must be informed".to_string(),
                );
            }
        }

        match dto.languages {
            Some(languages) => {
                if languages.is_empty() {
                    validations.insert(
                        "languages".to_string(),
                        "At least one language must be informed".to_string(),
                    );
                } else {
                    let mut languages_entity: Vec<Language> =
                        Vec::with_capacity(languages.capacity());
                    for language_dto in languages.into_iter() {
                        match Language::try_from(language_dto) {
                            Ok(language) => {
                                languages_entity.push(language);
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    book.languages = languages_entity;
                }
            }
            None => {
                validations.insert(
                    "languages".to_string(),
                    "Book languages must be informed".to_string(),
                );
            }
        }

        book.edition = dto.edition;
        book.isbn = dto.isbn;
        book.year = dto.year;

        match dto.genres {
            Some(genres) => {
                if genres.is_empty() {
                    validations.insert(
                        "genres".to_string(),
                        "At least one genre must be informed".to_string(),
                    );
                } else {
                    let mut genres_entity: Vec<Genre> = Vec::with_capacity(genres.capacity());
                    for genre_dto in genres.into_iter() {
                        match Genre::try_from(genre_dto) {
                            Ok(genre) => {
                                genres_entity.push(genre);
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    book.genres = Some(genres_entity);
                }
            }
            None => {
                book.genres = None;
            }
        }

        book.cover = dto.cover;
        book.collection_id = dto.collection_id;

        match dto.location_id {
            Some(location_id) => book.location_id = location_id,
            None => {
                validations.insert(
                    "location_id".to_string(),
                    "Book must be at a location".to_string(),
                );
            }
        }

        match dto.user_id {
            Some(user_id) => book.user_id = user_id,
            None => {
                validations.insert(
                    "user_id".to_string(),
                    "Book must be related to an user".to_string(),
                );
            }
        }

        if !validations.is_empty() {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(book)
    }
}

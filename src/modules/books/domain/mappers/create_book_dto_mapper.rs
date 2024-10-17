use crate::modules::{
    books::domain::{
        dtos::create_book_dto::CreateBookDto,
        entities::{author::Author, book::Book, genre::Genre, language::Language},
    },
    shared::errors::detailed_api_error::DetailedAPIError,
};
use base64::prelude::*;
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
        match dto.publisher {
            Some(publisher) => {
                let candidate_publisher = publisher.trim();
                if candidate_publisher.is_empty() {
                    validations.insert(
                        "publisher".to_string(),
                        "O campo editora não pode ser vazio".to_string(),
                    );
                }
                book.publisher = candidate_publisher.to_string()
            }
            None => {
                validations.insert(
                    "publisher".to_string(),
                    "A editora deve ser informada".to_string(),
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

        match dto.cover {
            None => {
                book.cover = None;
            }
            Some(cover_to_be) => {
                if cover_to_be.starts_with("http://") || cover_to_be.starts_with("https://") {
                    book.cover = Some(cover_to_be)
                } else {
                    match BASE64_STANDARD.decode(cover_to_be) {
                        Ok(_decoded_image) => {
                            //TODO upload to S3 or something
                        }
                        Err(_error) => {
                            validations.insert(
                                "cover".to_string(),
                                "A capa do livro contém uma imagem inválida".to_string(),
                            );
                        }
                    }
                }
            }
        }
        book.collection_id = dto.collection_id;

        match dto.location_id {
            Some(location_id) => book.location_id = location_id,
            None => {
                validations.insert(
                    "location_id".to_string(),
                    "O livro deve estar em alguma localização".to_string(),
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
                msg: "Livro contém informações inválidas".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(book)
    }
}

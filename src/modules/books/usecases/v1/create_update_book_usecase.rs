use std::sync::Arc;

use crate::modules::{
    books::{
        domain::{
            dtos::{
                collection_dto::CollectionDto, complete_book_dto::CompleteBookDto,
                location_dto::LocationDto,
            },
            entities::book::{self, Book},
        },
        infra::repositories::{
            book_repository::BookRepository, book_repository_mysql::BookRepositoryMySQL,
            collection_repository::CollectionRepository,
            collection_repository_mysql::CollectionRepositoryMySQL,
            location_repository::LocationRepository,
            location_repository_mysql::LocationRepositoryMySQL,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct CreateUpdateBookUseCaseV1<T, U, V>
where
    T: BookRepository,
    U: CollectionRepository,
    V: LocationRepository,
{
    book_repository: Arc<T>,
    collection_repository: Arc<U>,
    location_repository: Arc<V>,
}

impl
    CreateUpdateBookUseCaseV1<
        BookRepositoryMySQL,
        CollectionRepositoryMySQL,
        LocationRepositoryMySQL,
    >
{
    pub fn new(
        book_repository: BookRepositoryMySQL,
        collection_repository: CollectionRepositoryMySQL,
        location_repository: LocationRepositoryMySQL,
    ) -> Self {
        Self {
            book_repository: Arc::new(book_repository),
            collection_repository: Arc::new(collection_repository),
            location_repository: Arc::new(location_repository),
        }
    }

    pub async fn create_update_book(
        &self,
        book_to_be_created: Book,
    ) -> Result<CompleteBookDto, APIError> {
        if book_to_be_created.id.is_some() {
            match self
                .book_repository
                .find_by_id(book_to_be_created.id.unwrap())
                .await
            {
                Ok(maybe_a_book) => {
                    if maybe_a_book.is_none() {
                        return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                            "Book not found".to_string(),
                            404,
                        )));
                    }
                }
                Err(error) => {
                    return Err(APIError::SimpleAPIError(SimpleAPIError {
                        msg: error.to_string(),
                        code: 500,
                    }))
                }
            }
        }

        match self
            .book_repository
            .find_by_title(&book_to_be_created.title)
            .await
        {
            Ok(duplicated_book) => {
                if let Some(duplicated_book) = duplicated_book {
                    if duplicated_book.id != book_to_be_created.id {
                        return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                            "Já existe um livro com o mesmo nome".to_string(),
                            409,
                        )));
                    }
                }
            }
            Err(error) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: error.to_string(),
                    code: 500,
                }))
            }
        }

        if book_to_be_created.collection_id.is_some() {
            match self
                .collection_repository
                .find_by_id(book_to_be_created.collection_id.unwrap())
                .await
            {
                Ok(maybe_a_collection) => {
                    if maybe_a_collection.is_none() {
                        return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                            "The informed book's collection does not exist".to_string(),
                            404,
                        )));
                    }
                }
                Err(error) => {
                    return Err(APIError::SimpleAPIError(SimpleAPIError {
                        msg: error.to_string(),
                        code: 500,
                    }))
                }
            }
        }

        match self
            .location_repository
            .find_by_id(book_to_be_created.location_id)
            .await
        {
            Ok(maybe_a_location) => {
                if maybe_a_location.is_none() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "The informed book's location does not exist".to_string(),
                        404,
                    )));
                }
            }
            Err(error) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: error.to_string(),
                    code: 500,
                }))
            }
        }

        let saved_book;

        match self.book_repository.save(&book_to_be_created).await {
            Ok(t) => match t {
                Some(returned_book) => saved_book = returned_book,
                None => {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "Failed to load book info".to_string(),
                        500,
                    )))
                }
            },
            Err(e) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    e.to_string(),
                    500,
                )))
            }
        }

        let location_id = saved_book.location_id;
        let collection_id = saved_book.collection_id;

        let mut dto = match CompleteBookDto::try_from(saved_book) {
            Ok(converted_book) => converted_book,
            Err(e) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    e.to_string(),
                    500,
                )))
            }
        };

        if collection_id.is_some() {
            match self
                .collection_repository
                .find_by_id(collection_id.unwrap())
                .await
            {
                Ok(maybe_collection) => match maybe_collection {
                    Some(returned_collection) => {
                        dto.collection = Some(CollectionDto::from(returned_collection));
                    }
                    None => {
                        dto.collection = None;
                    }
                },
                Err(e) => {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        e.to_string(),
                        500,
                    )))
                }
            }
        }

        match self.location_repository.find_by_id(location_id).await {
            Ok(maybe_location) => match maybe_location {
                Some(found_location) => dto.location = LocationDto::from(found_location),
                None => {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "Book location not found".to_string(),
                        404,
                    )))
                }
            },
            Err(e) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    e.to_string(),
                    500,
                )))
            }
        }

        Ok(dto)
    }
}

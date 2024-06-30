use std::sync::Arc;

use crate::modules::{
    books::{
        domain::entities::collection::Collection,
        infra::repositories::{
            book_repository::BookRepository, book_repository_mysql::BookRepositoryMySQL,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct CreateBookUseCaseV1<T>
where
    T: BookRepository,
{
    book_repository: Arc<T>,
}

impl CreateBookUseCaseV1<BookRepositoryMySQL> {
    pub fn new(book_repository: BookRepositoryMySQL) -> Self {
        Self {
            book_repository: Arc::new(book_repository),
        }
    }

    pub async fn create_book(
        &self,
        book_to_be_created: Collection,
    ) -> Result<Collection, APIError> {
        match self
            .book_repository
            .find_by_name_and_user_id(&book_to_be_created.name, book_to_be_created.user_id)
            .await
        {
            Ok(duplicated_book_name) => {
                if duplicated_book_name.is_some() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "There is already an book with the given name".to_string(),
                        409,
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

        match self.book_repository.save(&book_to_be_created).await {
            Ok(t) => match t {
                Some(returned_book) => Ok(returned_book),
                None => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to load created book info".to_string(),
                    500,
                ))),
            },
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                e.to_string(),
                500,
            ))),
        }
    }
}

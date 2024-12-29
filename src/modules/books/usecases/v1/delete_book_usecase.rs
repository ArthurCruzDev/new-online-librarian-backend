use std::sync::Arc;

use crate::modules::{
    books::{
        domain::dtos::complete_book_dto::CompleteBookDto,
        infra::repositories::{
            book_repository::BookRepository, book_repository_mysql::BookRepositoryMySQL,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct DeleteBookUseCaseV1<T>
where
    T: BookRepository,
{
    book_repository: Arc<T>,
}

impl DeleteBookUseCaseV1<BookRepositoryMySQL> {
    pub fn new(book_repository: BookRepositoryMySQL) -> Self {
        Self {
            book_repository: Arc::new(book_repository),
        }
    }

    pub async fn delete_book_by_id(&self, user_id: u64, book_id: u64) -> Result<(), APIError> {
        match self.book_repository.delete_by_id(user_id, book_id).await {
            Ok(delete_book) => {
                if delete_book {
                    Ok(())
                } else {
                    Err(APIError::SimpleAPIError(SimpleAPIError {
                        msg: "Book not found".to_string(),
                        code: 404,
                    }))
                }
            }
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError {
                msg: e.to_string(),
                code: 500,
            })),
        }
    }
}

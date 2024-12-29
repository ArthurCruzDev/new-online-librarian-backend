use std::sync::Arc;

use crate::modules::{
    books::{
        domain::dtos::complete_book_dto::CompleteBookDto,
        infra::repositories::{
            book_repository::BookRepository, book_repository_mysql::BookRepositoryMySQL,
        },
    },
    shared::{
        domain::dtos::paginated_dto::PaginatedDto,
        errors::{simple_api_error::SimpleAPIError, APIError},
    },
};

pub struct FindAllBooksFromUserUseCaseV1<T>
where
    T: BookRepository,
{
    book_repository: Arc<T>,
}

impl FindAllBooksFromUserUseCaseV1<BookRepositoryMySQL> {
    pub fn new(book_repository: BookRepositoryMySQL) -> Self {
        Self {
            book_repository: Arc::new(book_repository),
        }
    }

    pub async fn find_all_from_user(
        &self,
        user_id: u64,
        page: Option<i64>,
        page_size: Option<i64>,
        collection_id: Option<i64>,
        location_id: Option<i64>,
        query: Option<String>,
    ) -> Result<PaginatedDto<CompleteBookDto>, APIError> {
        let converted_page: u64;
        if page.is_some() {
            if page.unwrap() < 1 {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: "Requested page must have a value greater than one".to_string(),
                    code: 400,
                }));
            } else {
                converted_page = u64::from_ne_bytes(page.unwrap().to_ne_bytes());
            }
        } else {
            converted_page = 1;
        }
        let converted_page_size: u64;
        if page_size.is_some() {
            if page_size.unwrap() < 1 {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: "Requested page size must have a value greater than one".to_string(),
                    code: 400,
                }));
            } else {
                converted_page_size = u64::from_ne_bytes(page_size.unwrap().to_ne_bytes());
            }
        } else {
            converted_page_size = 10
        }

        match self
            .book_repository
            .find_all_by_user_id_as_complete_book_dto(
                user_id,
                converted_page,
                converted_page_size,
                collection_id,
                location_id,
                query.clone(),
            )
            .await
        {
            Ok(found_books) => Ok(found_books),
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError {
                msg: e.to_string(),
                code: 500,
            })),
        }
    }
}

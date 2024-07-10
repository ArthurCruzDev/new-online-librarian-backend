use crate::modules::{
    books::{
        domain::{dtos::create_book_dto::CreateBookDto, entities::book::Book},
        infra::repositories::{
            book_repository_mysql::BookRepositoryMySQL,
            collection_repository_mysql::CollectionRepositoryMySQL,
            location_repository_mysql::LocationRepositoryMySQL,
        },
        usecases::v1::{
            create_book_usecase::CreateBookUseCaseV1,
            find_all_books_from_user_usecase::FindAllBooksFromUserUseCaseV1,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::domain::dtos::authed_user::AuthedUser,
};
use actix_web::{get, post, web, HttpResponse, Scope};
use serde::Deserialize;

pub struct BookControllerV1 {
    create_book_usecase: CreateBookUseCaseV1<
        BookRepositoryMySQL,
        CollectionRepositoryMySQL,
        LocationRepositoryMySQL,
    >,
    get_all_books_from_user_usecase: FindAllBooksFromUserUseCaseV1<BookRepositoryMySQL>,
}

impl BookControllerV1 {
    pub fn new(
        book_repository: BookRepositoryMySQL,
        collection_repository: CollectionRepositoryMySQL,
        location_repository: LocationRepositoryMySQL,
    ) -> Self {
        BookControllerV1 {
            create_book_usecase: CreateBookUseCaseV1::new(
                book_repository.clone(),
                collection_repository.clone(),
                location_repository.clone(),
            ),
            get_all_books_from_user_usecase: FindAllBooksFromUserUseCaseV1::new(
                book_repository.clone(),
            ),
        }
    }
}

#[post("")]
async fn create_book(
    book_controller: web::Data<BookControllerV1>,
    create_book_dto: web::Json<CreateBookDto>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    let book = match Book::try_from(create_book_dto.0) {
        Ok(converted_book) => converted_book,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };

    if authed_user.id.is_some_and(|id| id != book.user_id) {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "User doesn't have permission to create this resource".to_string(),
            403,
        )));
    }

    match book_controller.create_book_usecase.create_book(book).await {
        Ok(book) => HttpResponse::Created().json(web::Json(book)),
        Err(error) => HttpResponse::from(error),
    }
}

#[derive(Deserialize)]
pub struct GetAllBooksParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[get("")]
async fn get_all_books_paginated(
    book_controller: web::Data<BookControllerV1>,
    params: web::Query<GetAllBooksParams>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match book_controller
        .get_all_books_from_user_usecase
        .find_all_from_user(authed_user.id.unwrap(), params.page, params.page_size)
        .await
    {
        Ok(books_page) => HttpResponse::Created().json(web::Json(books_page)),
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_book_scope() -> Scope {
    web::scope("/v1/books")
        .service(create_book)
        .service(get_all_books_paginated)
}

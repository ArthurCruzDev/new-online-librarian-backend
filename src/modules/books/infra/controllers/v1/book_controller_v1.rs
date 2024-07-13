use crate::modules::{
    books::{
        domain::{dtos::create_book_dto::CreateBookDto, entities::book::Book},
        infra::repositories::{
            book_repository_mysql::BookRepositoryMySQL,
            collection_repository_mysql::CollectionRepositoryMySQL,
            location_repository_mysql::LocationRepositoryMySQL,
        },
        usecases::v1::{
            create_update_book_usecase::CreateUpdateBookUseCaseV1,
            delete_book_usecase::DeleteBookUseCaseV1,
            find_all_books_from_user_usecase::FindAllBooksFromUserUseCaseV1,
            find_book_by_id_usecase::FindBookByIDUseCaseV1,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::domain::dtos::authed_user::AuthedUser,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use serde::Deserialize;

pub struct BookControllerV1 {
    create_update_book_usecase: CreateUpdateBookUseCaseV1<
        BookRepositoryMySQL,
        CollectionRepositoryMySQL,
        LocationRepositoryMySQL,
    >,
    get_all_books_from_user_usecase: FindAllBooksFromUserUseCaseV1<BookRepositoryMySQL>,
    find_book_by_id_usecase: FindBookByIDUseCaseV1<BookRepositoryMySQL>,
    delete_book_by_id_usecase: DeleteBookUseCaseV1<BookRepositoryMySQL>,
}

impl BookControllerV1 {
    pub fn new(
        book_repository: BookRepositoryMySQL,
        collection_repository: CollectionRepositoryMySQL,
        location_repository: LocationRepositoryMySQL,
    ) -> Self {
        BookControllerV1 {
            create_update_book_usecase: CreateUpdateBookUseCaseV1::new(
                book_repository.clone(),
                collection_repository.clone(),
                location_repository.clone(),
            ),
            get_all_books_from_user_usecase: FindAllBooksFromUserUseCaseV1::new(
                book_repository.clone(),
            ),
            find_book_by_id_usecase: FindBookByIDUseCaseV1::new(book_repository.clone()),
            delete_book_by_id_usecase: DeleteBookUseCaseV1::new(book_repository.clone()),
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

    match book_controller
        .create_update_book_usecase
        .create_update_book(book)
        .await
    {
        Ok(book) => HttpResponse::Created().json(web::Json(book)),
        Err(error) => HttpResponse::from(error),
    }
}

#[put("/{book_id}")]
async fn update_book(
    book_controller: web::Data<BookControllerV1>,
    path: web::Path<(u64,)>,
    update_book_dto: web::Json<CreateBookDto>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    let mut book = match Book::try_from(update_book_dto.0) {
        Ok(converted_book) => converted_book,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };

    if authed_user.id.is_some_and(|id| id != book.user_id) {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "User doesn't have permission to update this resource".to_string(),
            403,
        )));
    }

    book.id = Some(path.into_inner().0);

    match book_controller
        .create_update_book_usecase
        .create_update_book(book)
        .await
    {
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
        Ok(books_page) => HttpResponse::Ok().json(web::Json(books_page)),
        Err(error) => HttpResponse::from(error),
    }
}

#[get("/{book_id}")]
async fn get_book_by_id(
    book_controller: web::Data<BookControllerV1>,
    path: web::Path<(u64,)>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match book_controller
        .find_book_by_id_usecase
        .find_book_by_id(authed_user.id.unwrap(), path.into_inner().0)
        .await
    {
        Ok(books_page) => HttpResponse::Ok().json(web::Json(books_page)),
        Err(error) => HttpResponse::from(error),
    }
}

#[delete("/{book_id}")]
async fn delete_book_by_id(
    book_controller: web::Data<BookControllerV1>,
    path: web::Path<(u64,)>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match book_controller
        .delete_book_by_id_usecase
        .delete_book_by_id(authed_user.id.unwrap(), path.into_inner().0)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_book_scope() -> Scope {
    web::scope("/v1/books")
        .service(create_book)
        .service(get_all_books_paginated)
        .service(get_book_by_id)
        .service(update_book)
        .service(delete_book_by_id)
}

use crate::modules::{
    books::{
        domain::{
            dtos::{
                create_collection_dto::CreateCollectionDto,
                find_all_collections_from_user_dto::FindAllCollectionsFromUserDto,
            },
            entities::collection::Collection,
        },
        infra::repositories::collection_repository_mysql::CollectionRepositoryMySQL,
        usecases::v1::{
            create_collection_usecase::CreateCollectionUseCaseV1,
            delete_collection_usecase::DeleteCollectionUseCaseV1,
            find_all_collection_from_user_usecase::FindAllCollectionFromUserUseCaseV1,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::domain::dtos::authed_user::AuthedUser,
};
use actix_web::{delete, get, post, web, HttpResponse, Scope};

pub struct CollectionControllerV1 {
    create_collection_usecase: CreateCollectionUseCaseV1<CollectionRepositoryMySQL>,
    delete_collection_usecase: DeleteCollectionUseCaseV1<CollectionRepositoryMySQL>,
    find_all_collection_from_user_usecase:
        FindAllCollectionFromUserUseCaseV1<CollectionRepositoryMySQL>,
}

impl CollectionControllerV1 {
    pub fn new(collection_repository: CollectionRepositoryMySQL) -> Self {
        CollectionControllerV1 {
            create_collection_usecase: CreateCollectionUseCaseV1::new(
                collection_repository.clone(),
            ),
            delete_collection_usecase: DeleteCollectionUseCaseV1::new(
                collection_repository.clone(),
            ),
            find_all_collection_from_user_usecase: FindAllCollectionFromUserUseCaseV1::new(
                collection_repository.clone(),
            ),
        }
    }
}

#[post("")]
async fn create_collection(
    collection_controller: web::Data<CollectionControllerV1>,
    create_collection_dto: web::Json<CreateCollectionDto>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    let collection = match Collection::try_from(create_collection_dto.0) {
        Ok(converted_collection) => converted_collection,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };

    if authed_user.id.is_some_and(|id| id != collection.user_id) {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "User doesn't have permission to create this resource".to_string(),
            403,
        )));
    }

    match collection_controller
        .create_collection_usecase
        .create_collection(collection)
        .await
    {
        Ok(collection) => HttpResponse::Created().json(web::Json(collection)),
        Err(error) => HttpResponse::from(error),
    }
}

#[get("")]
async fn get_all_collections_from_user(
    collection_controller: web::Data<CollectionControllerV1>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match collection_controller
        .find_all_collection_from_user_usecase
        .find_all_collection_from_user(authed_user.id.unwrap())
        .await
    {
        Ok(collections) => {
            HttpResponse::Ok().json(web::Json(FindAllCollectionsFromUserDto { collections }))
        }
        Err(error) => HttpResponse::from(error),
    }
}

#[delete("/{collection_id}")]
async fn delete_collection(
    collection_controller: web::Data<CollectionControllerV1>,
    path_variables: web::Path<u64>,
    authed_user: AuthedUser,
) -> HttpResponse {
    let path_collection_id = path_variables.into_inner();

    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match collection_controller
        .delete_collection_usecase
        .delete_collection(path_collection_id, authed_user.id.unwrap())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_collection_scope() -> Scope {
    web::scope("/v1/collections")
        .service(create_collection)
        .service(delete_collection)
        .service(get_all_collections_from_user)
    // .service(get_collection)
}

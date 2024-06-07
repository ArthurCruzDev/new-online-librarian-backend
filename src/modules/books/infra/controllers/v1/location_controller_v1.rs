use crate::modules::{
    books::{
        domain::{
            dtos::{
                create_location_dto::CreateLocationDto,
                find_all_locations_from_user_dto::FindAllLocationsFromUserDto,
            },
            entities::location::Location,
        },
        infra::repositories::location_repository_mysql::LocationRepositoryMySQL,
        usecases::v1::{
            create_location_usecase::CreateLocationUseCaseV1,
            delete_location_usecase::DeleteLocationUseCaseV1,
            find_all_location_from_user_usecase::{self, FindAllLocationFromUserUseCaseV1},
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::domain::dtos::authed_user::AuthedUser,
};
use actix_web::{delete, get, post, web, HttpResponse, Scope};

pub struct LocationControllerV1 {
    create_location_usecase: CreateLocationUseCaseV1<LocationRepositoryMySQL>,
    delete_location_usecase: DeleteLocationUseCaseV1<LocationRepositoryMySQL>,
    find_all_location_from_user_usecase: FindAllLocationFromUserUseCaseV1<LocationRepositoryMySQL>,
}

impl LocationControllerV1 {
    pub fn new(location_repository: LocationRepositoryMySQL) -> Self {
        LocationControllerV1 {
            create_location_usecase: CreateLocationUseCaseV1::new(location_repository.clone()),
            delete_location_usecase: DeleteLocationUseCaseV1::new(location_repository.clone()),
            find_all_location_from_user_usecase: FindAllLocationFromUserUseCaseV1::new(
                location_repository.clone(),
            ),
        }
    }
}

#[post("")]
async fn create_location(
    location_controller: web::Data<LocationControllerV1>,
    create_location_dto: web::Json<CreateLocationDto>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    let location = match Location::try_from(create_location_dto.0) {
        Ok(converted_location) => converted_location,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };

    if authed_user.id.is_some_and(|id| id != location.user_id) {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "User doesn't have permission to create this resource".to_string(),
            403,
        )));
    }

    match location_controller
        .create_location_usecase
        .create_location(location)
        .await
    {
        Ok(location) => HttpResponse::Created().json(web::Json(location)),
        Err(error) => HttpResponse::from(error),
    }
}

#[get("")]
async fn get_all_locations_from_user(
    location_controller: web::Data<LocationControllerV1>,
    authed_user: AuthedUser,
) -> HttpResponse {
    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match location_controller
        .find_all_location_from_user_usecase
        .find_all_location_from_user(authed_user.id.unwrap())
        .await
    {
        Ok(locations) => {
            HttpResponse::Ok().json(web::Json(FindAllLocationsFromUserDto { locations }))
        }
        Err(error) => HttpResponse::from(error),
    }
}

#[delete("/{location_id}")]
async fn delete_location(
    location_controller: web::Data<LocationControllerV1>,
    path_variables: web::Path<u64>,
    authed_user: AuthedUser,
) -> HttpResponse {
    let path_location_id = path_variables.into_inner();

    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    match location_controller
        .delete_location_usecase
        .delete_location(path_location_id, authed_user.id.unwrap())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_location_scope() -> Scope {
    web::scope("/v1/locations")
        .service(create_location)
        .service(delete_location)
        .service(get_all_locations_from_user)
    // .service(get_location)
}

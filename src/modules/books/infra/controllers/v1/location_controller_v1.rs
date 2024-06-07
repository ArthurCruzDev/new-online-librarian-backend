use crate::modules::{
    books::{
        domain::{dtos::create_location_dto::CreateLocationDto, entities::location::Location},
        infra::repositories::location_repository_mysql::LocationRepositoryMySQL,
        usecases::v1::{
            create_location_usecase::CreateLocationUseCaseV1,
            delete_location_usecase::DeleteLocationUseCaseV1,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::domain::dtos::authed_user::AuthedUser,
};
use actix_web::{body::BoxBody, delete, post, web, HttpResponse, Scope};

pub struct LocationControllerV1 {
    create_location_usecase: CreateLocationUseCaseV1<LocationRepositoryMySQL>,
    delete_location_usecase: DeleteLocationUseCaseV1<LocationRepositoryMySQL>,
}

impl LocationControllerV1 {
    pub fn new(location_repository: LocationRepositoryMySQL) -> Self {
        LocationControllerV1 {
            create_location_usecase: CreateLocationUseCaseV1::new(location_repository.clone()),
            delete_location_usecase: DeleteLocationUseCaseV1::new(location_repository.clone()),
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

// #[get("/{location_id}")]
// async fn get_location(
//     location_controller: web::Data<LocationControllerV1>,
//     path_variables: web::Path<u64>,
//     authed_location: AuthedLocation,
// ) -> HttpResponse {
//     let path_location_id = path_variables.into_inner();

//     if authed_location.id.is_none() {
//         return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
//             "This action requires authentication".to_string(),
//             401,
//         )));
//     }

//     if authed_location.id.is_some_and(|id| id != path_location_id) {
//         return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
//             "User doesn't have permission to access this resource".to_string(),
//             403,
//         )));
//     }

//     match location_controller
//         .get_location_info_usecase
//         .get_location_info(path_location_id)
//         .await
//     {
//         Ok(location) => match CreatedLocationDto::try_from(location) {
//             Ok(created_location_dto) => {
//                 HttpResponse::Created().json(web::Json(created_location_dto))
//             }
//             Err(e) => HttpResponse::from(APIError::SimpleAPIError(e)),
//         },
//         Err(error) => HttpResponse::from(error),
//     }
// }

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
    // .service(get_location)
}

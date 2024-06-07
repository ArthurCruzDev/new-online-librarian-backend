use crate::modules::{
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::{
        domain::{
            dtos::{
                authed_user::AuthedUser, create_user_dto::CreateUserDto,
                created_user_dto::CreatedUserDto,
            },
            entities::user::User,
        },
        infra::repositories::user_repository_mysql::UserRepositoryMySQL,
        usecases::v1::{create_user::CreateUserUseCaseV1, get_user_info::GetUserInfoUseCaseV1},
    },
};
use actix_web::{get, post, web, HttpResponse, Scope};

pub struct UserControllerV1 {
    create_user_usecase: CreateUserUseCaseV1<UserRepositoryMySQL>,
    get_user_info_usecase: GetUserInfoUseCaseV1<UserRepositoryMySQL>,
}

impl UserControllerV1 {
    pub fn new(user_repository: UserRepositoryMySQL) -> Self {
        UserControllerV1 {
            create_user_usecase: CreateUserUseCaseV1::new(user_repository.clone()),
            get_user_info_usecase: GetUserInfoUseCaseV1::new(user_repository.clone()),
        }
    }
}

#[post("")]
async fn create_user(
    user_controller: web::Data<UserControllerV1>,
    create_user_dto: web::Json<CreateUserDto>,
) -> HttpResponse {
    let user = match User::try_from(create_user_dto.0) {
        Ok(converted_user) => converted_user,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };
    match user_controller.create_user_usecase.create_user(user).await {
        Ok(user) => match CreatedUserDto::try_from(user) {
            Ok(created_user_dto) => HttpResponse::Created().json(web::Json(created_user_dto)),
            Err(e) => HttpResponse::from(APIError::SimpleAPIError(e)),
        },
        Err(error) => HttpResponse::from(error),
    }
}

#[get("/{user_id}")]
async fn get_user(
    user_controller: web::Data<UserControllerV1>,
    path_variables: web::Path<u64>,
    authed_user: AuthedUser,
) -> HttpResponse {
    let path_user_id = path_variables.into_inner();

    if authed_user.id.is_none() {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "This action requires authentication".to_string(),
            401,
        )));
    }

    if authed_user.id.is_some_and(|id| id != path_user_id) {
        return HttpResponse::from(APIError::SimpleAPIError(SimpleAPIError::new(
            "User doesn't have permission to access this resource".to_string(),
            403,
        )));
    }

    match user_controller
        .get_user_info_usecase
        .get_user_info(path_user_id)
        .await
    {
        Ok(user) => match CreatedUserDto::try_from(user) {
            Ok(created_user_dto) => HttpResponse::Created().json(web::Json(created_user_dto)),
            Err(e) => HttpResponse::from(APIError::SimpleAPIError(e)),
        },
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_user_scope() -> Scope {
    web::scope("/v1/users")
        .service(create_user)
        .service(get_user)
}

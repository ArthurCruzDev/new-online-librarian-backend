use crate::{
    configuration::TokenSettings,
    modules::{
        shared::errors::APIError,
        users::{
            domain::{dtos::login_user_dto::LoginUserDto, entities::user::User},
            infra::repositories::user_repository_mysql::UserRepositoryMySQL,
            usecases::v1::login_user::LoginUserUseCaseV1,
        },
    },
};
use actix_web::{post, web, HttpResponse, Scope};

pub struct AuthControllerV1 {
    token_settings: TokenSettings,
    login_user_usecase: LoginUserUseCaseV1<UserRepositoryMySQL>,
}

impl AuthControllerV1 {
    pub fn new(user_repository: UserRepositoryMySQL, token_settings: TokenSettings) -> Self {
        AuthControllerV1 {
            token_settings,
            login_user_usecase: LoginUserUseCaseV1::new(user_repository),
        }
    }
}

#[post("/login")]
async fn login_user(
    auth_controller: web::Data<AuthControllerV1>,
    login_user_dto: web::Json<LoginUserDto>,
) -> HttpResponse {
    let user = match User::try_from(login_user_dto.0) {
        Ok(converted_user) => converted_user,
        Err(e) => {
            return HttpResponse::from(APIError::DetailedAPIError(e));
        }
    };
    match auth_controller
        .login_user_usecase
        .login_user(user, &auth_controller.token_settings)
        .await
    {
        Ok(token_user_dto) => HttpResponse::Ok().json(web::Json(token_user_dto)),
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_user_scope() -> Scope {
    web::scope("/v1/auth").service(login_user)
}

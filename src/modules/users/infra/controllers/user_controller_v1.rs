use crate::modules::users::{
    domain::dtos::create_user_dto::CreateUserDto,
    infra::repositories::user_repository_mysql::UserRepositoryMySQL,
    usecases::v1::create_user::CreateUserUseCaseV1,
};
use actix_web::{post, web, HttpResponse, Scope};

pub struct UserControllerV1 {
    create_user_usecase: CreateUserUseCaseV1<UserRepositoryMySQL>,
}

impl UserControllerV1 {
    pub fn new(user_repository: UserRepositoryMySQL) -> Self {
        UserControllerV1 {
            create_user_usecase: CreateUserUseCaseV1::new(user_repository),
        }
    }
}

#[post("")]
async fn create_user(
    user_controller: web::Data<UserControllerV1>,
    create_user_dto: web::Json<CreateUserDto>,
) -> HttpResponse {
    match user_controller
        .create_user_usecase
        .create_user(create_user_dto.0)
        .await
    {
        Ok(user) => match user.id {
            Some(id) => HttpResponse::Created().body(format!("{}", id)),
            None => HttpResponse::InternalServerError()
                .body("Failed to retrieve created user id".to_string()),
        },
        Err(error) => HttpResponse::from(error),
    }
}

pub fn get_user_scope() -> Scope {
    web::scope("/v1/users").service(create_user)
}

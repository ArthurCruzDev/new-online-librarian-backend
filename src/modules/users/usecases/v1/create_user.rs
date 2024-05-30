use std::sync::Arc;

use crate::modules::{
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::{
        domain::entities::user::User,
        infra::repositories::{
            user_repository::UserRepository, user_repository_mysql::UserRepositoryMySQL,
        },
    },
};

pub struct CreateUserUseCaseV1<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
}

impl CreateUserUseCaseV1<UserRepositoryMySQL> {
    pub fn new(user_repository: UserRepositoryMySQL) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }

    pub async fn create_user(&self, user_to_be_created: User) -> Result<User, APIError> {
        match self
            .user_repository
            .find_by_email(&user_to_be_created.email)
            .await
        {
            Ok(duplicated_email_user) => {
                if duplicated_email_user.is_some() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "There is already an account with this e-mail".to_string(),
                        409,
                    )));
                }
            }
            Err(error) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: error.to_string(),
                    code: 500,
                }))
            }
        }

        match self.user_repository.save(&user_to_be_created).await {
            Ok(t) => match t {
                Some(returned_user) => Ok(returned_user),
                None => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to load created user info".to_string(),
                    500,
                ))),
            },
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                e.to_string(),
                500,
            ))),
        }
    }
}

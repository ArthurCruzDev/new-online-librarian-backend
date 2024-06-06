use std::sync::Arc;

use tracing_log::log::error;

use crate::modules::{
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
    users::{
        domain::entities::user::User,
        infra::repositories::{
            user_repository::UserRepository, user_repository_mysql::UserRepositoryMySQL,
        },
    },
};

pub struct GetUserInfoUseCaseV1<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
}

impl GetUserInfoUseCaseV1<UserRepositoryMySQL> {
    pub fn new(user_repository: UserRepositoryMySQL) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }

    pub async fn get_user_info(&self, user_id: u64) -> Result<User, APIError> {
        match self.user_repository.find_by_id(user_id).await {
            Ok(user) => match user {
                Some(found_user) => Ok(found_user),
                None => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "User not found".to_string(),
                    404,
                ))),
            },
            Err(error) => {
                error!("Failed to retrieve user info: {}", error);
                Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to retrieve user info".to_string(),
                    500,
                )))
            }
        }
    }
}

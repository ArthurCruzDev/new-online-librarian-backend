use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::error;

use crate::{
    configuration::TokenSettings,
    modules::{
        shared::errors::{simple_api_error::SimpleAPIError, APIError},
        users::{
            domain::{
                dtos::{claims_dto::ClaimsDto, token_user_dto::TokenUserDto},
                entities::user::User,
            },
            infra::repositories::{
                user_repository::UserRepository, user_repository_mysql::UserRepositoryMySQL,
            },
        },
    },
};

pub struct LoginUserUseCaseV1<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
}

impl LoginUserUseCaseV1<UserRepositoryMySQL> {
    pub fn new(user_repository: UserRepositoryMySQL) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }

    pub async fn login_user(
        &self,
        user: User,
        token_settings: &TokenSettings,
    ) -> Result<TokenUserDto, APIError> {
        let option_user_from_db = match self.user_repository.find_by_email(&user.email).await {
            Ok(found_user) => found_user,
            Err(error) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    format!("Failed to retrieve user from DB: {}", error),
                    500,
                )));
            }
        };
        let user_from_db = match option_user_from_db {
            Some(returned_user) => returned_user,
            None => {
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Invalid credentials".to_string(),
                    403,
                )));
            }
        };

        let password_hash = match PasswordHash::new(&user_from_db.password) {
            Ok(pass_hash) => pass_hash,
            Err(error) => {
                error!("{}", error);
                return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to parse password from DB".to_string(),
                    500,
                )));
            }
        };

        match Argon2::default().verify_password(user.password.as_bytes(), &password_hash) {
            Ok(_) => {
                let expiration_time =
                    chrono::offset::Utc::now() + Duration::seconds(token_settings.expiration_time);

                let claims_dto = ClaimsDto {
                    exp: expiration_time.timestamp(),
                };

                let generated_token = match encode(
                    &Header::default(),
                    &claims_dto,
                    &EncodingKey::from_secret(token_settings.secret.as_ref()),
                ) {
                    Ok(token) => token,
                    Err(error) => {
                        error!("{}", error);
                        return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                            "Unexpected error while authorizing".to_string(),
                            500,
                        )));
                    }
                };
                Ok(TokenUserDto {
                    access_token: generated_token,
                    token_type: "Bearer".to_string(),
                    expires_in: expiration_time.timestamp(),
                    scope: None,
                })
            }
            Err(error) => {
                error!("{}", error);
                Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Invalid credentials".to_string(),
                    403,
                )))
            }
        }
    }
}

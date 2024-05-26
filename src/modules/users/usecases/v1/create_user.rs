use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use email_address::EmailAddress;
use std::{collections::HashMap, sync::Arc};

use crate::modules::{
    shared::errors::{
        detailed_api_error::DetailedAPIError, simple_api_error::SimpleAPIError, APIError,
    },
    users::{
        domain::{dtos::create_user_dto::CreateUserDto, entities::user::User},
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

    pub async fn create_user(&self, create_user_dto: CreateUserDto) -> Result<User, APIError> {
        let user = match User::try_from(create_user_dto) {
            Ok(converted_user) => converted_user,
            Err(e) => {
                return Err(APIError::DetailedAPIError(e));
            }
        };

        match self.user_repository.save(&user).await {
            Ok(t) => Ok(t),
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                e.to_string(),
                500,
            ))),
        }
    }
}

impl TryFrom<CreateUserDto> for User {
    type Error = DetailedAPIError;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        let mut user = User::default();
        let mut errors = false;
        let mut validations: HashMap<String, String> = HashMap::default();
        match dto.name {
            Some(name) => {
                let candidate_name = name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "Full name must not be empty".to_string(),
                    );
                    errors = true;
                }
                if candidate_name.len() <= 2 || candidate_name.split_whitespace().count() <= 1 {
                    validations
                        .insert("name".to_string(), "Full name must be informed".to_string());
                    errors = true;
                }
            }
            None => {
                validations.insert("name".to_string(), "Name not informed".to_string());
                errors = true;
            }
        }

        match dto.email {
            Some(email) => {
                let candidate_email = email.trim();
                if candidate_email.is_empty() {
                    validations.insert("email".to_string(), "Email must not be empty".to_string());
                    errors = true;
                }
                if EmailAddress::is_valid(candidate_email) {
                    user.email = candidate_email.to_string();
                } else {
                    validations.insert(
                        "email".to_string(),
                        "informed email is not valid".to_string(),
                    );
                    errors = true;
                }
            }
            None => {
                validations.insert("email".to_string(), "Email not informed".to_string());
                errors = true;
            }
        }

        match dto.password {
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                match argon2.hash_password(password.as_bytes(), &salt) {
                    Ok(hashed_password) => {
                        user.password = hashed_password.to_string();
                    }
                    Err(e) => {
                        return Err(DetailedAPIError {
                            msg: e.to_string(),
                            code: 500,
                            field_validations: None,
                        });
                    }
                }
            }
            None => {
                validations.insert("passoword".to_string(), "Password not informed".to_string());
                errors = true;
            }
        }
        if errors {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }
        Ok(user)
    }
}

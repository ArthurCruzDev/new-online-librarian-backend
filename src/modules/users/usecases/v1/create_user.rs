use anyhow::{anyhow, bail};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::sync::Arc;

use crate::modules::{
    shared::errors::api_error::APIError,
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
                return Err(APIError::new(e.to_string(), 400));
            }
        };

        match self.user_repository.save(&user).await {
            Ok(t) => Ok(t),
            Err(e) => Err(APIError::new(e.to_string(), 500)),
        }
    }
}

impl TryFrom<CreateUserDto> for User {
    type Error = anyhow::Error;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        let mut user = User::default();

        match dto.name {
            Some(name) => user.name = name,
            None => {
                bail!("Name not informed");
            }
        }

        match dto.email {
            Some(email) => user.email = email,
            None => {
                bail!("Email not informed");
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
                        anyhow!(e);
                    }
                }
            }
            None => {
                bail!("Password not informed");
            }
        }

        Ok(user)
    }
}

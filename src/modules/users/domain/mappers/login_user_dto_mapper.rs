use std::collections::HashMap;

use crate::modules::{
    shared::errors::detailed_api_error::DetailedAPIError,
    users::domain::{dtos::login_user_dto::LoginUserDto, entities::user::User},
};

impl TryFrom<LoginUserDto> for User {
    type Error = DetailedAPIError;

    fn try_from(dto: LoginUserDto) -> Result<Self, Self::Error> {
        let mut user = User::default();
        let mut errors = false;
        let mut validations: HashMap<String, String> = HashMap::default();

        match dto.email {
            Some(email) => {
                let candidate_email = email.trim();
                if candidate_email.is_empty() {
                    validations.insert("email".to_string(), "Email must not be empty".to_string());
                    errors = true;
                }
                user.email = candidate_email.to_string();
            }
            None => {
                validations.insert("email".to_string(), "Email not informed".to_string());
                errors = true;
            }
        }

        match dto.password {
            Some(password) => {
                let candidate_password = password.trim();

                if candidate_password.is_empty() {
                    validations.insert(
                        "password".to_string(),
                        "Password must not be empty".to_string(),
                    );
                    errors = true;
                }

                user.password = candidate_password.to_string();
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

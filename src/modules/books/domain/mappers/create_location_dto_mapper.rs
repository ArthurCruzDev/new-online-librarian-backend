use std::collections::HashMap;

use crate::modules::{
    books::domain::{dtos::create_location_dto::CreateLocationDto, entities::location::Location},
    shared::errors::detailed_api_error::DetailedAPIError,
};

impl TryFrom<CreateLocationDto> for Location {
    type Error = DetailedAPIError;

    fn try_from(dto: CreateLocationDto) -> Result<Self, Self::Error> {
        let mut location = Location::default();
        let mut errors = false;
        let mut validations: HashMap<String, String> = HashMap::default();
        match dto.name {
            Some(name) => {
                let candidate_name = name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "Location name must not be empty".to_string(),
                    );
                    errors = true;
                }
                location.name = candidate_name.to_string();
            }
            None => {
                errors = true;
                validations.insert(
                    "name".to_string(),
                    "Location name must be informed".to_string(),
                );
            }
        }

        match dto.user_id {
            Some(user_id) => location.user_id = user_id,
            None => {
                errors = true;
                validations.insert(
                    "user_id".to_string(),
                    "Location must be related to an user".to_string(),
                );
            }
        }

        if errors {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(location)
    }
}

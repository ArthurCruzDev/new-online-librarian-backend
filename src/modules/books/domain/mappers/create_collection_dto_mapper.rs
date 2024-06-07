use std::collections::HashMap;

use crate::modules::{
    books::domain::{
        dtos::create_collection_dto::CreateCollectionDto, entities::collection::Collection,
    },
    shared::errors::detailed_api_error::DetailedAPIError,
};

impl TryFrom<CreateCollectionDto> for Collection {
    type Error = DetailedAPIError;

    fn try_from(dto: CreateCollectionDto) -> Result<Self, Self::Error> {
        let mut collection = Collection::default();
        let mut errors = false;
        let mut validations: HashMap<String, String> = HashMap::default();
        match dto.name {
            Some(name) => {
                let candidate_name = name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "Collection name must not be empty".to_string(),
                    );
                    errors = true;
                }
                collection.name = candidate_name.to_string();
            }
            None => {
                errors = true;
                validations.insert(
                    "name".to_string(),
                    "Collection name must be informed".to_string(),
                );
            }
        }

        match dto.user_id {
            Some(user_id) => collection.user_id = user_id,
            None => {
                errors = true;
                validations.insert(
                    "user_id".to_string(),
                    "Collection must be related to an user".to_string(),
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

        Ok(collection)
    }
}

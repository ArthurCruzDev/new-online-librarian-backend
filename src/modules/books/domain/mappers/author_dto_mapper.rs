use std::collections::HashMap;

use crate::modules::{
    books::domain::{dtos::author_dto::AuthorDto, entities::author::Author},
    shared::errors::detailed_api_error::DetailedAPIError,
};

impl TryFrom<AuthorDto> for Author {
    type Error = DetailedAPIError;

    fn try_from(dto: AuthorDto) -> Result<Self, Self::Error> {
        let mut author = Author::default();
        let mut validations: HashMap<String, String> = HashMap::default();

        match dto.name {
            Some(author_dto_name) => {
                let candidate_name = author_dto_name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "author name must not be empty".to_string(),
                    );
                }
                author.name = candidate_name.to_string();
            }
            None => {
                validations.insert(
                    "name".to_string(),
                    "author name must be informed".to_string(),
                );
            }
        }

        author.url = dto.url;

        if !validations.is_empty() {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(author)
    }
}

use std::collections::HashMap;

use crate::modules::{
    books::domain::{dtos::genre_dto::GenreDto, entities::genre::Genre},
    shared::errors::detailed_api_error::DetailedAPIError,
};

impl TryFrom<GenreDto> for Genre {
    type Error = DetailedAPIError;

    fn try_from(dto: GenreDto) -> Result<Self, Self::Error> {
        let mut genre = Genre::default();
        let mut validations: HashMap<String, String> = HashMap::default();

        match dto.name {
            Some(genre_dto_name) => {
                let candidate_name = genre_dto_name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "genre name must not be empty".to_string(),
                    );
                }
                genre.name = candidate_name.to_string();
            }
            None => {
                validations.insert(
                    "name".to_string(),
                    "genre name must be informed".to_string(),
                );
            }
        }

        if !validations.is_empty() {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(genre)
    }
}

impl From<Genre> for GenreDto {
    fn from(entity: Genre) -> Self {
        GenreDto {
            name: Some(entity.name),
        }
    }
}

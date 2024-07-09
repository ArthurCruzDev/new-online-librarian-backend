use std::collections::HashMap;

use crate::modules::{
    books::domain::{dtos::language_dto::LanguageDto, entities::language::Language},
    shared::errors::detailed_api_error::DetailedAPIError,
};

impl TryFrom<LanguageDto> for Language {
    type Error = DetailedAPIError;

    fn try_from(dto: LanguageDto) -> Result<Self, Self::Error> {
        let mut language = Language::default();
        let mut validations: HashMap<String, String> = HashMap::default();

        match dto.name {
            Some(author_dto_name) => {
                let candidate_name = author_dto_name.trim();
                if candidate_name.is_empty() {
                    validations.insert(
                        "name".to_string(),
                        "language name must not be empty".to_string(),
                    );
                }
                language.name = candidate_name.to_string();
            }
            None => {
                validations.insert(
                    "name".to_string(),
                    "language name must be informed".to_string(),
                );
            }
        }

        language.code = dto.code;

        if !validations.is_empty() {
            return Err(DetailedAPIError {
                msg: "Request contains invalid data".to_string(),
                code: 400,
                field_validations: Some(validations),
            });
        }

        Ok(language)
    }
}

impl From<Language> for LanguageDto {
    fn from(entity: Language) -> Self {
        LanguageDto {
            name: Some(entity.name),
            code: entity.code,
        }
    }
}

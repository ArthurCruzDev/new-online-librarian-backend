use std::sync::Arc;

use crate::modules::{
    books::{
        domain::entities::location::Location,
        infra::repositories::{
            location_repository::LocationRepository,
            location_repository_mysql::LocationRepositoryMySQL,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct FindAllLocationFromUserUseCaseV1<T>
where
    T: LocationRepository,
{
    location_repository: Arc<T>,
}

impl FindAllLocationFromUserUseCaseV1<LocationRepositoryMySQL> {
    pub fn new(location_repository: LocationRepositoryMySQL) -> Self {
        Self {
            location_repository: Arc::new(location_repository),
        }
    }

    pub async fn find_all_location_from_user(
        &self,
        authed_user_id: u64,
    ) -> Result<Vec<Location>, APIError> {
        match self
            .location_repository
            .find_all_by_user_id(authed_user_id)
            .await
        {
            Ok(found_location_option) => Ok(found_location_option),
            Err(error) => Err(APIError::SimpleAPIError(SimpleAPIError {
                msg: error.to_string(),
                code: 500,
            })),
        }
    }
}

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
pub struct CreateLocationUseCaseV1<T>
where
    T: LocationRepository,
{
    location_repository: Arc<T>,
}

impl CreateLocationUseCaseV1<LocationRepositoryMySQL> {
    pub fn new(location_repository: LocationRepositoryMySQL) -> Self {
        Self {
            location_repository: Arc::new(location_repository),
        }
    }

    pub async fn create_location(
        &self,
        location_to_be_created: Location,
    ) -> Result<Location, APIError> {
        match self
            .location_repository
            .find_by_name_and_user_id(&location_to_be_created.name, location_to_be_created.user_id)
            .await
        {
            Ok(duplicated_location_name) => {
                if duplicated_location_name.is_some() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "Já existe uma localização com esse nome.".to_string(),
                        409,
                    )));
                }
            }
            Err(error) => {
                return Err(APIError::SimpleAPIError(SimpleAPIError {
                    msg: error.to_string(),
                    code: 500,
                }))
            }
        }

        match self.location_repository.save(&location_to_be_created).await {
            Ok(t) => match t {
                Some(returned_location) => Ok(returned_location),
                None => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to load created location info".to_string(),
                    500,
                ))),
            },
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                e.to_string(),
                500,
            ))),
        }
    }
}

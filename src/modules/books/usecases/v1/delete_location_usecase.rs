use std::sync::Arc;

use crate::modules::{
    books::infra::repositories::{
        location_repository::LocationRepository, location_repository_mysql::LocationRepositoryMySQL,
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct DeleteLocationUseCaseV1<T>
where
    T: LocationRepository,
{
    location_repository: Arc<T>,
}

impl DeleteLocationUseCaseV1<LocationRepositoryMySQL> {
    pub fn new(location_repository: LocationRepositoryMySQL) -> Self {
        Self {
            location_repository: Arc::new(location_repository),
        }
    }

    pub async fn delete_location(
        &self,
        location_to_be_delete: u64,
        authed_user_id: u64,
    ) -> Result<(), APIError> {
        match self
            .location_repository
            .find_by_id(location_to_be_delete)
            .await
        {
            Ok(found_location_option) => {
                if found_location_option.is_none() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "Location not found".to_string(),
                        404,
                    )));
                } else if authed_user_id != found_location_option.unwrap().user_id {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "User doesn't have permission to delete this resource".to_string(),
                        403,
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

        match self
            .location_repository
            .delete_by_id(location_to_be_delete)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                e.to_string(),
                500,
            ))),
        }
    }
}

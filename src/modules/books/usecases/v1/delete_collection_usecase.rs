use std::sync::Arc;

use crate::modules::{
    books::infra::repositories::{
        collection_repository::CollectionRepository,
        collection_repository_mysql::CollectionRepositoryMySQL,
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};

pub struct DeleteCollectionUseCaseV1<T>
where
    T: CollectionRepository,
{
    collection_repository: Arc<T>,
}

impl DeleteCollectionUseCaseV1<CollectionRepositoryMySQL> {
    pub fn new(collection_repository: CollectionRepositoryMySQL) -> Self {
        Self {
            collection_repository: Arc::new(collection_repository),
        }
    }

    pub async fn delete_collection(
        &self,
        collection_to_be_delete: u64,
        authed_user_id: u64,
    ) -> Result<(), APIError> {
        match self
            .collection_repository
            .find_by_id(collection_to_be_delete)
            .await
        {
            Ok(found_collection_option) => {
                if found_collection_option.is_none() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "Collection not found".to_string(),
                        404,
                    )));
                } else if authed_user_id != found_collection_option.unwrap().user_id {
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
            .collection_repository
            .delete_by_id(collection_to_be_delete)
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

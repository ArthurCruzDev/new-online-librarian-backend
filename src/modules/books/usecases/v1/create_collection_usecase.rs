use std::sync::Arc;

use crate::modules::{
    books::{
        domain::entities::collection::Collection,
        infra::repositories::{
            collection_repository::CollectionRepository,
            collection_repository_mysql::CollectionRepositoryMySQL,
        },
    },
    shared::errors::{simple_api_error::SimpleAPIError, APIError},
};
pub struct CreateCollectionUseCaseV1<T>
where
    T: CollectionRepository,
{
    collection_repository: Arc<T>,
}

impl CreateCollectionUseCaseV1<CollectionRepositoryMySQL> {
    pub fn new(collection_repository: CollectionRepositoryMySQL) -> Self {
        Self {
            collection_repository: Arc::new(collection_repository),
        }
    }

    pub async fn create_collection(
        &self,
        collection_to_be_created: Collection,
    ) -> Result<Collection, APIError> {
        match self
            .collection_repository
            .find_by_name_and_user_id(
                &collection_to_be_created.name,
                collection_to_be_created.user_id,
            )
            .await
        {
            Ok(duplicated_collection_name) => {
                if duplicated_collection_name.is_some() {
                    return Err(APIError::SimpleAPIError(SimpleAPIError::new(
                        "There is already an collection with the given name".to_string(),
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

        match self
            .collection_repository
            .save(&collection_to_be_created)
            .await
        {
            Ok(t) => match t {
                Some(returned_collection) => Ok(returned_collection),
                None => Err(APIError::SimpleAPIError(SimpleAPIError::new(
                    "Failed to load created collection info".to_string(),
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

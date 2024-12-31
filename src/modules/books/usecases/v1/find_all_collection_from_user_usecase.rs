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

pub struct FindAllCollectionFromUserUseCaseV1<T>
where
    T: CollectionRepository,
{
    collection_repository: Arc<T>,
}

impl FindAllCollectionFromUserUseCaseV1<CollectionRepositoryMySQL> {
    pub fn new(collection_repository: CollectionRepositoryMySQL) -> Self {
        Self {
            collection_repository: Arc::new(collection_repository),
        }
    }

    pub async fn find_all_collection_from_user(
        &self,
        authed_user_id: u64,
    ) -> Result<Vec<Collection>, APIError> {
        match self
            .collection_repository
            .find_all_by_user_id(authed_user_id)
            .await
        {
            Ok(found_collection_option) => Ok(found_collection_option),
            Err(error) => Err(APIError::SimpleAPIError(SimpleAPIError {
                msg: error.to_string(),
                code: 500,
            })),
        }
    }
}

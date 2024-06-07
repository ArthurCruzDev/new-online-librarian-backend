use crate::modules::books::domain::entities::location::Location;
use sqlx::Error;
use std::future::Future;

pub trait LocationRepository {
    fn save(
        &self,
        location: &Location,
    ) -> impl Future<Output = Result<Option<Location>, Error>> + Send;
    fn find_by_id(&self, id: u64) -> impl Future<Output = Result<Option<Location>, Error>> + Send;
    fn find_by_name_and_user_id(
        &self,
        name: &str,
        user_id: u64,
    ) -> impl Future<Output = Result<Option<Location>, Error>> + Send;
    fn find_all_by_user_id(
        &self,
        user_id: u64,
    ) -> impl Future<Output = Result<Vec<Location>, Error>> + Send;
    fn delete_by_id(&self, id: u64) -> impl Future<Output = Result<(), Error>> + Send;
}

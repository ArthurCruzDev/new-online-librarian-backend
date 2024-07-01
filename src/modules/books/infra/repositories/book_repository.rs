use futures_util::Future;
use sqlx::Error;

use crate::modules::books::domain::entities::book::Book;

pub trait BookRepository {
    fn save(&self, location: &Book) -> impl Future<Output = Result<Option<Book>, Error>> + Send;
    fn find_by_id(&self, id: u64) -> impl Future<Output = Result<Option<Book>, Error>> + Send;
    fn find_all_by_user_id(
        &self,
        user_id: u64,
    ) -> impl Future<Output = Result<Vec<Book>, Error>> + Send;
    fn delete_by_id(&self, id: u64) -> impl Future<Output = Result<(), Error>> + Send;
}

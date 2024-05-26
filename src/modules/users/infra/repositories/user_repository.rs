use crate::modules::users::domain::entities::user::User;
use sqlx::Error;

pub trait UserRepository {
    fn save(&self, user: &User) -> impl std::future::Future<Output = Result<User, Error>> + Send;
    fn find_by_id(&self, id: u64) -> impl std::future::Future<Output = Result<User, Error>> + Send;
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<User, Error>> + Send;
    fn delete_by_id(&self, id: u64) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

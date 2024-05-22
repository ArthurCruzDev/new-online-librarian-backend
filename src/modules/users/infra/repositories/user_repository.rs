use crate::modules::users::domain::entities::user::User;
use sqlx::Error;

pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<User, Error>;
    async fn find_by_id(&self, id: u64) -> Result<User, Error>;
    async fn find_by_email(&self, email: &str) -> Result<User, Error>;
    async fn delete_by_id(&self, id: u64) -> Result<(), Error>;
}

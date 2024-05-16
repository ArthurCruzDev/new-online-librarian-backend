use actix_web::web;
use sqlx::MySqlPool;

use super::user_repository::UserRepository;
use crate::modules::users::domain::entities::user::User;
pub struct UserRepositoryMySQL {
    connection: web::Data<MySqlPool>,
}

impl UserRepository for UserRepositoryMySQL {
    async fn save(&self, user: &User) -> Result<User, sqlx::Error> {
        match user.id {
            Some(_) => {
                let insert_result = sqlx::query!(
                    r#"
                    INSERT INTO users (id, email, password, email_token, name, profile_picture, created_at, active)
                    VALUES (DEFAULT, ?, ?, ?, ?, ?, DEFAULT, DEFAULT)
                    "#,
                    user.email,
                    user.password,
                    user.email_token,
                    user.name,
                    user.profile_picture
                ).execute(self.connection.get_ref()).await;

                match insert_result {
                    Ok(result) => {
                        let new_user_id = result.last_insert_id();
                        Ok(self.find_by_id(new_user_id).await?)
                    }
                    Err(e) => Err(e),
                }
            }

            None => Ok(User::default()),
        }
    }

    async fn find_by_id(&self, id: u64) -> Result<User, sqlx::Error> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        todo!()
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), sqlx::Error> {
        todo!()
    }
}

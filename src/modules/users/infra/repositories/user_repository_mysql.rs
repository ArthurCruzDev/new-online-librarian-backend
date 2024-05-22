use std::sync::Arc;

use sqlx::MySqlPool;

use super::user_repository::UserRepository;
use crate::modules::users::domain::entities::user::User;
pub struct UserRepositoryMySQL {
    connection: Arc<MySqlPool>,
}

impl UserRepositoryMySQL {
    pub fn new(db_pool: Arc<MySqlPool>) -> Self {
        UserRepositoryMySQL {
            connection: db_pool.clone(),
        }
    }
}

impl UserRepository for UserRepositoryMySQL {
    async fn save(&self, user: &User) -> Result<User, sqlx::Error> {
        match user.id {
            Some(_) => Ok(User::default()),
            None => {
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
                ).execute(self.connection.as_ref()).await;
                match insert_result {
                    Ok(result) => {
                        let new_user_id = result.last_insert_id();
                        tracing::info!("ID do usuário gerado: {}", new_user_id);
                        Ok(self.find_by_id(new_user_id).await?)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    async fn find_by_id(&self, id: u64) -> Result<User, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM users u
            WHERE u.id = ?
            "#,
            id
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(User {
                id: Some(result.id),
                name: result.name,
                email: result.email,
                password: result.password,
                email_token: result.email_token,
                profile_picture: result.profile_picture,
                created_at: result.created_at,
                active: result.active != 0,
            }),
            Err(error) => Err(error),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        todo!()
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), sqlx::Error> {
        todo!()
    }
}

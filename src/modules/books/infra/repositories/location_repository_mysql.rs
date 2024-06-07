use sqlx::MySqlPool;
use std::sync::Arc;

use crate::modules::books::domain::entities::location::Location;

use super::location_repository::LocationRepository;

#[derive(Clone)]
pub struct LocationRepositoryMySQL {
    connection: Arc<MySqlPool>,
}

impl LocationRepositoryMySQL {
    pub fn new(db_pool: Arc<MySqlPool>) -> Self {
        LocationRepositoryMySQL {
            connection: db_pool.clone(),
        }
    }
}

impl LocationRepository for LocationRepositoryMySQL {
    async fn save(&self, location: &Location) -> Result<Option<Location>, sqlx::Error> {
        match location.id {
            Some(_) => todo!(),
            None => {
                let insert_result = sqlx::query!(
                    r#"
                    INSERT INTO locations (id, name, user_id)
                    VALUES (DEFAULT, ?, ?)
                    "#,
                    location.name,
                    location.user_id,
                )
                .execute(self.connection.as_ref())
                .await;
                match insert_result {
                    Ok(result) => {
                        let new_location_id = result.last_insert_id();
                        tracing::info!("Generated location ID: {}", new_location_id);
                        match self.find_by_id(new_location_id).await {
                            Ok(location_option) => match location_option {
                                Some(location) => Ok(Some(location)),
                                None => Ok(None),
                            },
                            Err(error) => Err(error),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Location>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM locations u
            WHERE u.id = ?
            "#,
            id
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(Some(Location {
                id: Some(result.id),
                name: result.name,
                user_id: result.user_id,
            })),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(error),
            },
        }
    }

    async fn find_by_name_and_user_id(
        &self,
        name: &str,
        user_id: u64,
    ) -> Result<Option<Location>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM locations u
            WHERE u.user_id = ?
                AND u.name = ?
            "#,
            user_id,
            name
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(Some(Location {
                id: Some(result.id),
                name: result.name,
                user_id: result.user_id,
            })),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(error),
            },
        }
    }

    async fn find_all_by_user_id(&self, user_id: u64) -> Result<Vec<Location>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM locations u
            WHERE u.user_id = ?
            "#,
            user_id,
        )
        .fetch_all(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(result
                .into_iter()
                .map(|item| Location {
                    id: Some(item.id),
                    name: item.name,
                    user_id: item.user_id,
                })
                .collect()),
            Err(error) => Err(error),
        }
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            DELETE
            FROM locations u
            WHERE u.id = ?
            "#,
            id,
        )
        .execute(self.connection.as_ref())
        .await;
        match query_result {
            Ok(_result) => Ok(()),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(()),
                _ => Err(error),
            },
        }
    }
}

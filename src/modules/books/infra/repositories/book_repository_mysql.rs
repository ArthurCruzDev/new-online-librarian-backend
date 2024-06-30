use sqlx::MySqlPool;
use std::sync::Arc;

use crate::modules::books::domain::entities::book::Book;

use super::book_repository::BookRepository;

#[derive(Clone)]
pub struct BookRepositoryMySQL {
    connection: Arc<MySqlPool>,
}

impl BookRepositoryMySQL {
    pub fn new(db_pool: Arc<MySqlPool>) -> Self {
        BookRepositoryMySQL {
            connection: db_pool.clone(),
        }
    }
}

impl BookRepository for BookRepositoryMySQL {
    async fn save(&self, book: &Book) -> Result<Option<Book>, sqlx::Error> {
        match book.id {
            Some(_) => todo!(),
            None => {
                let mut genres_string = String::new();

                match book.genres {
                    Some(book_genres) => {
                        genres_string = serde_json::to_string(&book_genres).unwrap()
                    }
                    None => {}
                }

                let insert_result = sqlx::query!(
                    r#"
                    INSERT INTO books (
                        id, 
                        title, 
                        authors, 
                        publisher, 
                        languages, 
                        edition, 
                        isbn, 
                        year, 
                        genres, 
                        cover, 
                        collection_id, 
                        location_id, 
                        user_id)
                    VALUES (DEFAULT, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    book.title,
                    serde_json::to_string(&book.authors).unwrap(),
                    book.publisher,
                    serde_json::to_string(&book.languages).unwrap(),
                    book.edition,
                    book.isbn,
                    book.year,
                    genres_string,
                    book.cover,
                    book.collection_id,
                    book.location_id,
                    book.user_id
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

    async fn find_by_id(&self, id: u64) -> Result<Option<Book>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM books u
            WHERE u.id = ?
            "#,
            id
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(Some(Book {
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
    ) -> Result<Option<Book>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM books u
            WHERE u.user_id = ?
                AND u.name = ?
            "#,
            user_id,
            name
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(Some(Book {
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

    async fn find_all_by_user_id(&self, user_id: u64) -> Result<Vec<Book>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM books u
            WHERE u.user_id = ?
            "#,
            user_id,
        )
        .fetch_all(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => Ok(result
                .into_iter()
                .map(|item| Book {
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
            FROM books u
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

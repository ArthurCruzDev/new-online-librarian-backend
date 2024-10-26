use serde_json::Value;
use sqlx::{types::Json, MySqlPool, Row};
use std::{str::FromStr, sync::Arc};
use tracing::info;

use crate::modules::{
    books::domain::{
        dtos::{
            collection_dto::CollectionDto, complete_book_dto::CompleteBookDto, genre_dto::GenreDto,
            location_dto::LocationDto,
        },
        entities::{book::Book, genre::Genre},
    },
    shared::domain::dtos::paginated_dto::PaginatedDto,
    users::domain::entities::user,
};

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

const COLLECTION_ID_CONDITIONAL: &str = "AND u.collection_id = ? \n";
const LOCATION_ID_CONDITIONAL: &str = "AND u.location_id = ? \n";
const QUERY_CONDTIONAL: &str = r#"
    AND (
            lower(u.title) LIKE CONCAT('%', ?, '%') 
        OR 	lower(u.authors->'$[*].name') LIKE CONCAT('%', ?, '%')
        OR	lower(u.publisher) LIKE CONCAT('%', ?, '%')
        OR	lower(u.isbn) LIKE CONCAT( ? , '%')
        OR 	lower(u.genres->'$[*].name') LIKE CONCAT('%', ?, '%')
    )
"#;

impl BookRepository for BookRepositoryMySQL {
    async fn save(&self, book: &Book) -> Result<Option<Book>, sqlx::Error> {
        let mut genres_string = String::new();

        match &book.genres {
            Some(book_genres) => genres_string = serde_json::to_string(&book_genres).unwrap(),
            None => {}
        }
        match book.id {
            Some(_) => {
                let update_result = sqlx::query!(
                    r#"
                    UPDATE books SET
                        title = ?, 
                        authors = ?, 
                        publisher = ?, 
                        languages = ?, 
                        edition = ?, 
                        isbn = ?, 
                        year = ?, 
                        genres = ?, 
                        cover = ?, 
                        collection_id = ?, 
                        location_id = ?
                    WHERE id = ? AND user_id = ?
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
                    book.id,
                    book.user_id
                )
                .execute(self.connection.as_ref())
                .await;
                match update_result {
                    Ok(_) => match self.find_by_id(book.id.unwrap()).await {
                        Ok(book_option) => match book_option {
                            Some(book) => Ok(Some(book)),
                            None => Ok(None),
                        },
                        Err(error) => Err(error),
                    },
                    Err(e) => Err(e),
                }
            }
            None => {
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
                        let new_book_id = result.last_insert_id();
                        tracing::info!("Generated book ID: {}", new_book_id);
                        match self.find_by_id(new_book_id).await {
                            Ok(book_option) => match book_option {
                                Some(book) => Ok(Some(book)),
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
            Ok(result) => {
                let mut genres: Option<Vec<Genre>> = None;
                if let Some(genre_value) = result.genres {
                    genres = Some(serde_json::from_value(genre_value).unwrap());
                }
                Ok(Some(Book {
                    id: Some(result.id),
                    title: result.title,
                    authors: serde_json::from_value(result.authors).unwrap(),
                    publisher: result.publisher,
                    languages: serde_json::from_value(result.languages).unwrap(),
                    edition: result.edition,
                    isbn: result.isbn,
                    year: result.year,
                    genres,
                    cover: result.cover,
                    collection_id: result.collection_id,
                    location_id: result.location_id,
                    user_id: result.user_id,
                }))
            }
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(error),
            },
        }
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Book>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT *
            FROM books u
            WHERE u.title = ?
            "#,
            title
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => {
                let mut genres: Option<Vec<Genre>> = None;
                if let Some(genre_value) = result.genres {
                    genres = Some(serde_json::from_value(genre_value).unwrap());
                }
                Ok(Some(Book {
                    id: Some(result.id),
                    title: result.title,
                    authors: serde_json::from_value(result.authors).unwrap(),
                    publisher: result.publisher,
                    languages: serde_json::from_value(result.languages).unwrap(),
                    edition: result.edition,
                    isbn: result.isbn,
                    year: result.year,
                    genres,
                    cover: result.cover,
                    collection_id: result.collection_id,
                    location_id: result.location_id,
                    user_id: result.user_id,
                }))
            }
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(error),
            },
        }
    }

    async fn find_all_by_user_id_as_complete_book_dto(
        &self,
        user_id: u64,
        page: u64,
        page_size: u64,
        collection_id: Option<i64>,
        location_id: Option<i64>,
        query: Option<String>,
    ) -> Result<PaginatedDto<CompleteBookDto>, sqlx::Error> {
        let query_hook = match query {
            Some(_) => QUERY_CONDTIONAL,
            None => "",
        };
        let collection_id_hook = match collection_id {
            Some(_) => COLLECTION_ID_CONDITIONAL,
            None => "",
        };
        let location_id_hook = match location_id {
            Some(_) => LOCATION_ID_CONDITIONAL,
            None => "",
        };

        let mut count_query = r#"
        SELECT COUNT(*) as n_books
            FROM books b
                INNER JOIN ( 
                    SELECT u.id
                        FROM books u
                        WHERE u.user_id = ?
                                "#
        .to_string();
        count_query.push_str(query_hook);
        count_query.push_str(location_id_hook);
        count_query.push_str(collection_id_hook);
        count_query.push_str(
            r#"
                ) as p USING (id)
                    INNER JOIN locations as l
                        ON l.id = b.location_id
                    LEFT JOIN collections as c
                        ON c.id = b.collection_id
        "#,
        );

        let mut count_query_ps = sqlx::query(&count_query).bind(user_id);
        if query.is_some() {
            let lowercase_query = query.clone().unwrap().to_lowercase();
            count_query_ps = count_query_ps
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone());
        }
        if location_id.is_some() {
            count_query_ps = count_query_ps.bind(location_id.unwrap());
        }
        if collection_id.is_some() {
            count_query_ps = count_query_ps.bind(collection_id.unwrap());
        }

        let couting_query_result = count_query_ps.fetch_one(self.connection.as_ref()).await;
        let n_of_books: u64 = match couting_query_result {
            Ok(counting_query_result_value) => {
                let n_books: i64 = counting_query_result_value.get(0);
                info!("{} books returned", n_books);
                u64::from_ne_bytes(n_books.to_ne_bytes())
            }
            Err(e) => return Err(e),
        };

        let mut main_query = r#"
        SELECT 
            b.id 'book_id',
            b.title 'book_title',
            b.authors 'book_authors',
            b.publisher 'book_publisher',
            b.languages 'book_languages',
            b.edition 'book_edition',
            b.isbn 'book_isbn',
            b.year 'book_year',
            b.genres 'book_genres',
            b.cover 'book_cover',
            b.user_id 'book_user_id',
            l.id 'location_id',
            l.name 'location_name',
            l.user_id 'location_user_id',
            c.id 'collection_id',
            c.name 'collection_name',
            c.user_id 'collection_user_id'
            FROM books b
                INNER JOIN ( 
                    SELECT u.id
                        FROM books u
                        WHERE u.user_id = ?
                                "#
        .to_string();
        main_query.push_str(query_hook);
        main_query.push_str(location_id_hook);
        main_query.push_str(collection_id_hook);
        main_query.push_str(
            r#"
                ORDER BY u.title ASC
                        LIMIT ? OFFSET ?
                ) as p USING (id)
            INNER JOIN locations as l
                ON l.id = b.location_id
            LEFT JOIN collections as c
                ON c.id = b.collection_id

        "#,
        );

        let mut query_ps = sqlx::query(&main_query).bind(user_id);

        if query.is_some() {
            let lowercase_query = query.clone().unwrap().to_lowercase();
            query_ps = query_ps
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone())
                .bind(lowercase_query.clone());
        }
        if location_id.is_some() {
            query_ps = query_ps.bind(location_id.unwrap());
        }
        if collection_id.is_some() {
            query_ps = query_ps.bind(collection_id.unwrap());
        }
        query_ps = query_ps.bind(page_size).bind((page - 1) * page_size);

        let query_result = query_ps.fetch_all(self.connection.as_ref()).await;
        match query_result {
            Ok(result) => {
                let items_vec = result
                    .into_iter()
                    .map(|item| {
                        let mut genres: Option<Vec<GenreDto>> = None;
                        let book_genre: Option<Value> = item.get(8);
                        if let Some(genre_value) = book_genre {
                            let genre_vec: Vec<Genre> =
                                serde_json::from_value(genre_value).unwrap();
                            let genre_dto_vec: Vec<GenreDto> =
                                genre_vec.into_iter().map(GenreDto::from).collect();
                            genres = Some(genre_dto_vec);
                        }
                        let mut collection: Option<CollectionDto> = None;
                        let book_collection_id: Option<u64> = item.get(14);
                        if book_collection_id.is_some() {
                            collection = Some(CollectionDto {
                                id: item.get(14),
                                name: item
                                    .get::<Option<String>, usize>(15)
                                    .unwrap_or("".to_string()),
                                user_id: item.get::<Option<u64>, usize>(16).unwrap(),
                            })
                        }
                        CompleteBookDto {
                            id: item.get(0),
                            title: item.get(1),
                            authors: serde_json::from_value(item.get(2)).unwrap(),
                            publisher: item.get(3),
                            languages: serde_json::from_value(item.get(4)).unwrap(),
                            edition: item.get(5),
                            isbn: item.get(6),
                            year: item.get(7),
                            genres,
                            cover: item.get(9),
                            collection,
                            location: LocationDto {
                                id: Some(item.get(11)),
                                name: item.get(12),
                                user_id: item.get(13),
                            },
                            user_id: item.get(10),
                        }
                    })
                    .collect();
                Ok(PaginatedDto {
                    page,
                    page_size,
                    total_items: n_of_books,
                    items: items_vec,
                })
            }
            Err(error) => Err(error),
        }
    }

    async fn delete_by_id(&self, user_id: u64, book_id: u64) -> Result<bool, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            DELETE
            FROM books u
            WHERE u.id = ? and u.user_id = ?
            "#,
            book_id,
            user_id
        )
        .execute(self.connection.as_ref())
        .await;
        match query_result {
            Ok(deleted_result) => {
                if deleted_result.rows_affected() > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(false),
                _ => Err(error),
            },
        }
    }

    async fn find_by_id_as_complete_book_dto(
        &self,
        user_id: u64,
        book_id: u64,
    ) -> Result<Option<CompleteBookDto>, sqlx::Error> {
        let query_result = sqlx::query!(
            r#"
            SELECT 
            b.id 'book_id',
            b.title 'book_title',
            b.authors 'book_authors',
            b.publisher 'book_publisher',
            b.languages 'book_languages',
            b.edition 'book_edition',
            b.isbn 'book_isbn',
            b.year 'book_year',
            b.genres 'book_genres',
            b.cover 'book_cover',
            b.user_id 'book_user_id',
            l.id 'location_id',
            l.name 'location_name',
            l.user_id 'location_user_id',
            c.id 'collection_id',
            c.name 'collection_name',
            c.user_id 'collection_user_id'
            FROM books b
            INNER JOIN locations as l
                ON l.id = b.location_id
            LEFT JOIN collections as c
                ON c.id = b.collection_id
            WHERE 
                b.user_id = ?
                AND b.id = ?
            "#,
            user_id,
            book_id
        )
        .fetch_one(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => {
                let mut genres: Option<Vec<GenreDto>> = None;
                if let Some(genre_value) = result.book_genres {
                    let genre_vec: Vec<Genre> = serde_json::from_value(genre_value).unwrap();
                    let genre_dto_vec: Vec<GenreDto> =
                        genre_vec.into_iter().map(GenreDto::from).collect();
                    genres = Some(genre_dto_vec);
                }
                let mut collection: Option<CollectionDto> = None;
                if result.collection_id.is_some() {
                    collection = Some(CollectionDto {
                        id: result.collection_id,
                        name: result.collection_name.unwrap_or("".to_string()),
                        user_id: result.collection_user_id.unwrap(),
                    })
                }
                Ok(Some(CompleteBookDto {
                    id: result.book_id,
                    title: result.book_title,
                    authors: serde_json::from_value(result.book_authors).unwrap(),
                    publisher: result.book_publisher,
                    languages: serde_json::from_value(result.book_languages).unwrap(),
                    edition: result.book_edition,
                    isbn: result.book_isbn,
                    year: result.book_year,
                    genres,
                    cover: result.book_cover,
                    collection,
                    location: LocationDto {
                        id: Some(result.location_id),
                        name: result.location_name,
                        user_id: result.location_user_id,
                    },
                    user_id: result.book_user_id,
                }))
            }
            Err(error) => match error {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(error),
            },
        }
    }
}

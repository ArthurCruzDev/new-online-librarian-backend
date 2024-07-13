use sqlx::MySqlPool;
use std::sync::Arc;

use crate::modules::{
    books::domain::{
        dtos::{
            collection_dto::CollectionDto, complete_book_dto::CompleteBookDto, genre_dto::GenreDto,
            location_dto::LocationDto,
        },
        entities::{book::Book, genre::Genre},
    },
    shared::domain::dtos::paginated_dto::PaginatedDto,
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

impl BookRepository for BookRepositoryMySQL {
    async fn save(&self, book: &Book) -> Result<Option<Book>, sqlx::Error> {
        match book.id {
            Some(_) => todo!(),
            None => {
                let mut genres_string = String::new();

                match &book.genres {
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
    ) -> Result<PaginatedDto<CompleteBookDto>, sqlx::Error> {
        let couting_query_result = sqlx::query!(
            r#"
            SELECT COUNT(*) as n_books
            FROM books u
            WHERE u.user_id = ?
            "#,
            user_id,
        )
        .fetch_one(self.connection.as_ref())
        .await;
        let n_of_books: u64 = match couting_query_result {
            Ok(counting_query_result_value) => {
                u64::from_ne_bytes(counting_query_result_value.n_books.to_ne_bytes())
            }
            Err(e) => return Err(e),
        };

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
                INNER JOIN ( 
                    SELECT u.id
                        FROM books u
                        WHERE u.user_id = ?
                        ORDER BY u.title ASC
                        LIMIT ? OFFSET ?
                ) as p USING (id)
            INNER JOIN locations as l
                ON l.id = b.location_id
            LEFT JOIN collections as c
                ON c.id = b.collection_id
            "#,
            user_id,
            page_size,
            (page - 1) * page_size
        )
        .fetch_all(self.connection.as_ref())
        .await;
        match query_result {
            Ok(result) => {
                let items_vec = result
                    .into_iter()
                    .map(|item| {
                        let mut genres: Option<Vec<GenreDto>> = None;
                        if let Some(genre_value) = item.book_genres {
                            let genre_vec: Vec<Genre> =
                                serde_json::from_value(genre_value).unwrap();
                            let genre_dto_vec: Vec<GenreDto> =
                                genre_vec.into_iter().map(GenreDto::from).collect();
                            genres = Some(genre_dto_vec);
                        }
                        let mut collection: Option<CollectionDto> = None;
                        if item.collection_id.is_some() {
                            collection = Some(CollectionDto {
                                id: item.collection_id,
                                name: item.collection_name.unwrap_or("".to_string()),
                                user_id: item.collection_user_id.unwrap(),
                            })
                        }
                        CompleteBookDto {
                            id: item.book_id,
                            title: item.book_title,
                            authors: serde_json::from_value(item.book_authors).unwrap(),
                            publisher: item.book_publisher,
                            languages: serde_json::from_value(item.book_languages).unwrap(),
                            edition: item.book_edition,
                            isbn: item.book_isbn,
                            year: item.book_year,
                            genres,
                            cover: item.book_cover,
                            collection,
                            location: LocationDto {
                                id: Some(item.location_id),
                                name: item.location_name,
                                user_id: item.location_user_id,
                            },
                            user_id: item.book_user_id,
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

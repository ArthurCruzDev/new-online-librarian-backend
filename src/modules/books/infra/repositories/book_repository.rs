use futures_util::Future;
use sqlx::Error;

use crate::modules::{
    books::domain::{dtos::complete_book_dto::CompleteBookDto, entities::book::Book},
    shared::domain::dtos::paginated_dto::PaginatedDto,
};

pub trait BookRepository {
    fn save(&self, location: &Book) -> impl Future<Output = Result<Option<Book>, Error>> + Send;
    fn find_by_id(&self, id: u64) -> impl Future<Output = Result<Option<Book>, Error>> + Send;
    fn find_by_title(
        &self,
        title: &str,
    ) -> impl Future<Output = Result<Option<Book>, Error>> + Send;
    fn find_all_by_user_id_as_complete_book_dto(
        &self,
        user_id: u64,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PaginatedDto<CompleteBookDto>, Error>> + Send;
    fn find_by_id_as_complete_book_dto(
        &self,
        user_id: u64,
        book_id: u64,
    ) -> impl Future<Output = Result<Option<CompleteBookDto>, Error>> + Send;
    fn delete_by_id(&self, id: u64) -> impl Future<Output = Result<(), Error>> + Send;
}

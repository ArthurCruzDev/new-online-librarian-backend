use crate::modules::books::domain::{
    dtos::collection_dto::CollectionDto, entities::collection::Collection,
};

impl From<Collection> for CollectionDto {
    fn from(entity: Collection) -> Self {
        CollectionDto {
            id: entity.id,
            name: entity.name,
            user_id: entity.user_id,
        }
    }
}

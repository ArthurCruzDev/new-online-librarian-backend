use crate::modules::books::domain::{
    dtos::location_dto::LocationDto, entities::location::Location,
};

impl From<Location> for LocationDto {
    fn from(entity: Location) -> Self {
        LocationDto {
            id: entity.id,
            name: entity.name,
            user_id: entity.user_id,
        }
    }
}

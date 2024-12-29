use serde::Serialize;

use crate::modules::books::domain::entities::collection::Collection;

#[derive(Debug, Default, Serialize)]
pub struct FindAllCollectionsFromUserDto {
    pub collections: Vec<Collection>,
}

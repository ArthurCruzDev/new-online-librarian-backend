use serde::Serialize;

use crate::modules::books::domain::entities::location::Location;

#[derive(Debug, Default, Serialize)]
pub struct FindAllLocationsFromUserDto {
    pub locations: Vec<Location>,
}

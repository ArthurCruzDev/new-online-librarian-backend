use crate::modules::{
    shared::errors::simple_api_error::SimpleAPIError,
    users::domain::{dtos::created_user_dto::CreatedUserDto, entities::user::User},
};

impl TryFrom<User> for CreatedUserDto {
    type Error = SimpleAPIError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let mut created_user_dto = CreatedUserDto::default();
        let parsing_error = SimpleAPIError::new("Generated user has no ID".to_string(), 500);
        match user.id {
            Some(id) => {
                created_user_dto.id = id;
            }
            None => {
                return Err(parsing_error);
            }
        }
        created_user_dto.email = user.email;
        created_user_dto.name = user.name;
        created_user_dto.profile_picture = user.profile_picture;

        Ok(created_user_dto)
    }
}

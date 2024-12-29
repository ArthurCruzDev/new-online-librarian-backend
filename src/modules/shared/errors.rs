use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};

use self::{detailed_api_error::DetailedAPIError, simple_api_error::SimpleAPIError};

pub mod detailed_api_error;
pub mod simple_api_error;

pub enum APIError {
    SimpleAPIError(SimpleAPIError),
    DetailedAPIError(DetailedAPIError),
}

impl From<APIError> for HttpResponse {
    fn from(value: APIError) -> Self {
        match value {
            APIError::SimpleAPIError(sae) => {
                let status_code =
                    StatusCode::from_u16(sae.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                HttpResponseBuilder::new(status_code).json(sae)
            }
            APIError::DetailedAPIError(dae) => {
                let status_code =
                    StatusCode::from_u16(dae.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                HttpResponseBuilder::new(status_code).json(dae)
            }
        }
    }
}

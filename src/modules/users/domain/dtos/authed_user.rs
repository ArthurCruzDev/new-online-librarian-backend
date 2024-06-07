use std::sync::Arc;

use actix_web::{http::header, Error, FromRequest, HttpRequest};
use futures_util::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing_log::log::{error, info};

use crate::configuration::TokenSettings;

use super::claims_dto::ClaimsDto;

pub struct AuthedUser {
    pub id: Option<u64>,
}

impl FromRequest for AuthedUser {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => match header.to_str().unwrap().strip_prefix("Bearer ") {
                Some(token) => token.to_string(),
                None => String::new(),
            },
            None => {
                return ok(AuthedUser { id: None });
            }
        };

        let token_settings = match req.app_data::<Arc<TokenSettings>>() {
            Some(tokensettings) => tokensettings,
            None => {
                error!("Failed to load token settings");
                return ok(AuthedUser { id: None });
            }
        };

        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        let token_id = match decode::<ClaimsDto>(
            token.as_str(),
            &DecodingKey::from_secret(token_settings.secret.as_ref()),
            &validation,
        ) {
            Ok(token_data) => token_data.claims.id,
            Err(err) => {
                error!("{}", err);
                return ok(AuthedUser { id: None });
            }
        };

        let authed_user = AuthedUser { id: Some(token_id) };

        ok(authed_user)
    }
}

use std::net::TcpListener;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::MySqlPool;
use tracing_actix_web::TracingLogger;
use tracing_log::log::{error, info};

use crate::configuration::TokenSettings;
use crate::modules::users::domain::dtos::claims_dto::ClaimsDto;
use crate::modules::users::infra::controllers::auth_controller_v1::{self, AuthControllerV1};
use crate::modules::users::infra::controllers::user_controller_v1::{self, UserControllerV1};
use crate::modules::users::infra::repositories::user_repository_mysql::UserRepositoryMySQL;
use crate::routes::health_check::health_check;

pub fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    token_settings: TokenSettings,
) -> Result<Server, std::io::Error> {
    let arc_db_pool = Arc::new(db_pool);
    let arc_token_settings = Arc::new(token_settings);
    let user_repository = UserRepositoryMySQL::new(arc_db_pool);
    let user_controller_v1 = web::Data::new(UserControllerV1::new(user_repository.clone()));
    let auth_controller_v1 = web::Data::new(AuthControllerV1::new(
        user_repository.clone(),
        arc_token_settings.clone(),
    ));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(user_controller_v1::get_user_scope())
            .service(auth_controller_v1::get_user_scope())
            .app_data(user_controller_v1.clone())
            .app_data(auth_controller_v1.clone())
            .app_data(arc_token_settings.clone())
            .app_data(user_repository.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

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
                info!("{}", err);
                return ok(AuthedUser { id: None });
            }
        };

        let authed_user = AuthedUser { id: Some(token_id) };

        ok(authed_user)
    }
}

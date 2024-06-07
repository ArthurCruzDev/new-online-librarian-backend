use std::net::TcpListener;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
use tracing_actix_web::TracingLogger;

use crate::configuration::TokenSettings;
use crate::modules::books::infra::controllers::v1::location_controller_v1::{
    self, LocationControllerV1,
};
use crate::modules::books::infra::repositories::location_repository_mysql::LocationRepositoryMySQL;
use crate::modules::users::infra::controllers::v1::auth_controller_v1::{self, AuthControllerV1};
use crate::modules::users::infra::controllers::v1::user_controller_v1::{self, UserControllerV1};
use crate::modules::users::infra::repositories::user_repository_mysql::UserRepositoryMySQL;
use crate::routes::health_check::health_check;

pub fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    token_settings: TokenSettings,
) -> Result<Server, std::io::Error> {
    let arc_db_pool = Arc::new(db_pool);
    let arc_token_settings = Arc::new(token_settings);

    let user_repository = UserRepositoryMySQL::new(arc_db_pool.clone());
    let location_repository = LocationRepositoryMySQL::new(arc_db_pool.clone());

    let user_controller_v1 = web::Data::new(UserControllerV1::new(user_repository.clone()));
    let auth_controller_v1 = web::Data::new(AuthControllerV1::new(
        user_repository.clone(),
        arc_token_settings.clone(),
    ));
    let location_controller_v1 =
        web::Data::new(LocationControllerV1::new(location_repository.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(user_controller_v1::get_user_scope())
            .service(auth_controller_v1::get_auth_scope())
            .service(location_controller_v1::get_location_scope())
            .app_data(user_controller_v1.clone())
            .app_data(auth_controller_v1.clone())
            .app_data(arc_token_settings.clone())
            .app_data(location_controller_v1.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

use std::net::TcpListener;
use std::sync::Arc;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::MySqlPool;
use tracing_actix_web::TracingLogger;

use crate::configuration::TokenSettings;
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
    let user_repository = UserRepositoryMySQL::new(arc_db_pool);
    let user_controller_v1 = web::Data::new(UserControllerV1::new(user_repository.clone()));
    let auth_controller_v1 = web::Data::new(AuthControllerV1::new(
        user_repository.clone(),
        token_settings,
    ));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(user_controller_v1::get_user_scope())
            .service(auth_controller_v1::get_user_scope())
            .app_data(user_controller_v1.clone())
            .app_data(auth_controller_v1.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

use std::net::TcpListener;
use std::sync::Arc;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::MySqlPool;
use tracing_actix_web::TracingLogger;

use crate::modules::users::infra::controllers::user_controller_v1::{
    get_user_scope, UserControllerV1,
};
use crate::modules::users::infra::repositories::user_repository_mysql::UserRepositoryMySQL;
use crate::routes::health_check::health_check;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    let arc_db_pool = Arc::new(db_pool);
    let user_repository = UserRepositoryMySQL::new(arc_db_pool);
    let user_controller_v1 = web::Data::new(UserControllerV1::new(user_repository));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(get_user_scope())
            .app_data(user_controller_v1.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::MySqlPool;
use tracing_actix_web::TracingLogger;

use crate::modules::users::usecases;
use crate::routes::health_check::health_check;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

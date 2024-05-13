use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::{MySqlConnection, MySqlPool};

use crate::routes::health_check::health_check;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    let server = HttpServer::new(move || App::new()
    .wrap(Logger::default())
    .route("/health_check", web::get().to(health_check))
    .app_data(connection.clone())
)
        .listen(listener)?
        .run();
    Ok(server)
}

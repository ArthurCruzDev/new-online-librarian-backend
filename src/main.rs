use std::net::TcpListener;

use new_online_librarian_backend::configuration::get_configuration;
use new_online_librarian_backend::startup::run;
use new_online_librarian_backend::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        MySqlPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to MySQL");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

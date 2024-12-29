use std::net::TcpListener;

use new_online_librarian_backend::configuration::get_configuration;
use new_online_librarian_backend::startup::run;
use new_online_librarian_backend::telemetry::{get_subscriber, init_subscriber};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newonlinelibrarian".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.connection_options());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, configuration.token)?.await
}

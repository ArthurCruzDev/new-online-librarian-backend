use new_online_librarian_backend::{configuration::get_configuration, startup::run};
use sqlx::MySqlPool;
use std::net::TcpListener;

pub struct TestApp{
    pub address: String,
    pub db_pool:MySqlPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = MySqlPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to MySql");

    let server = run(listener, connection_pool.clone())
    .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}



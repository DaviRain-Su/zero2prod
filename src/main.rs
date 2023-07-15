use anyhow::Result;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber)?;

    let configuration = get_configuration()?;
    let db_connection_str = configuration.database.connection_string();

    std::env::set_var("DATABASE_URL", db_connection_str.clone());

    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await?;

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool).await
}

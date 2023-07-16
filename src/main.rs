use anyhow::Result;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "trace".into(), std::io::stdout);
    init_subscriber(subscriber)?;

    let configuration = get_configuration()?;

    std::env::set_var(
        "DATABASE_URL",
        configuration.database.connection_string().expose_secret(),
    );

    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret()).await?;

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool).await
}

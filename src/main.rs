use anyhow::Result;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_subscriber::EnvFilter;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace,tokio=trace,hyper=trace");
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let configuration = get_configuration()?;
    let db_connection_str = configuration.database.connection_string();

    std::env::set_var("DATABASE_URL", db_connection_str.clone());

    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await?;

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool).await
}

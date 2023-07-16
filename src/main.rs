use anyhow::Result;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
// use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    // let subscriber = get_subscriber("zero2prod".into(), "trace".into(), std::io::stdout);
    // init_subscriber(subscriber)?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zero2prod=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

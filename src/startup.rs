use crate::configuration::{DatabaseSettings, Settings};
use anyhow::Result;
use axum::extract::Form;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::email_client::EmailClient;
use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::index;
use crate::routes::{subscribe, using_connection_pool_extractor};

pub fn get_connection_poll(database_configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database_configuration.with_db())
}

pub async fn build(configuration: &Settings) -> Result<()> {
    let connection_pool = get_connection_poll(&configuration.database);
    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .map_err(|_| anyhow::anyhow!("Invalid sender email address."))?;
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url.clone(),
        sender_email,
        configuration.email_client.authorization_token.clone(),
        // pass new argument from configuration
        timeout,
    )?;

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client).await?;

    Ok(())
}

pub async fn run(
    listener: TcpListener,
    conn_pool: PgPool,
    email_client: EmailClient,
) -> Result<()> {
    tracing::debug!("listening on {}", listener.local_addr()?);

    let email_client = Form(email_client);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/greet/:name", get(greet))
        .route("/health_check", get(health_check))
        .route(
            "/subscriptions",
            get(using_connection_pool_extractor).post(subscribe),
        )
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(conn_pool)
        .with_state(email_client); // ref: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs#L27

    // run it with hyper on localhost:3000
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

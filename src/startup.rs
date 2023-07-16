use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::index;
use crate::routes::{subscribe, using_connection_pool_extractor};

pub async fn run(listener: TcpListener, conn_pool: PgPool) -> Result<()> {
    tracing::debug!("listening on {}", listener.local_addr()?);

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
        .with_state(conn_pool); // ref: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs#L27

    // run it with hyper on localhost:3000
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

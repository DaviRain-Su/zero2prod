use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::Pool;
use sqlx::Postgres;
use std::net::TcpListener;

use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::index;
use crate::routes::subscribe;

pub async fn run(listener: TcpListener, conn_pool: Pool<Postgres>) -> Result<()> {
    tracing::info!("listener: {:?}", listener);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(conn_pool); // ref: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs#L27

    // run it with hyper on localhost:3000
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

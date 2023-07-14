use anyhow::Result;
use axum::routing::IntoMakeService;
use axum::{
    routing::{get, post},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

use routes::greet;
use routes::health_check;
use routes::index;
use routes::subscribe;

pub fn run(listener: TcpListener) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    tracing::info!("listener: {:?}", listener);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    // run it with hyper on localhost:3000
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}

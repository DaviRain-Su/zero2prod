use anyhow::Result;
use axum::routing::IntoMakeService;
use axum::{
    extract::{Form, Path},
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use serde::Deserialize;
use std::net::TcpListener;

async fn index() -> &'static str {
    "Hello, World!"
}

// `Path(name): Path<String>` 这一部分是 Axum 的 extractors。它们允许你从请求中提取数据。
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

async fn health_check() -> impl IntoResponse {
    ""
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

// Let's start simple: we always return a 200 OK
async fn subscriptions(Form(form): Form<FormData>) -> impl IntoResponse {
    // Here you can use the form data.
    format!("Received subscription from {} at {}", form.name, form.email)
}

pub fn run(listener: TcpListener) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    tracing::info!("listener: {:?}", listener);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    // run it with hyper on localhost:3000
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::health_check;

    #[tokio::test]
    async fn health_check_success() {
        let response = health_check().await;
        assert!(response.into_response().status().is_success());
    }
}

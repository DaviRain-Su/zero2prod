use anyhow::Result;
use axum::routing::IntoMakeService;
use axum::{extract::Path, response::IntoResponse, routing::get, Router, Server};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

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

pub fn run(address: &str) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    let addr: SocketAddr = address.parse()?;
    tracing_subscriber::fmt::init();

    println!("listening on {}", addr);

    // run it with hyper on localhost:3000
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

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

use anyhow::Result;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Hello, World!"
}

// `Path(name): Path<String>` 这一部分是 Axum 的 extractors。它们允许你从请求中提取数据。
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

async fn health_check() -> impl IntoResponse {
    "Server is running!"
}

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing_subscriber::fmt::init();

    println!("listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::health_check;

    #[tokio::test]
    async fn health_check_succeds() {
        let response = health_check().await;
        assert!(response.into_response().status().is_success());
    }
}

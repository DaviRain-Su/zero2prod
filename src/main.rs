use anyhow::Result;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Hello, World!"
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/:name", get(greet));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing_subscriber::fmt::init();

    println!("listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

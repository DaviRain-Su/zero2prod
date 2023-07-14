use axum::extract::Path;
use axum::response::IntoResponse;

// `Path(name): Path<String>` 这一部分是 Axum 的 extractors。它们允许你从请求中提取数据。
pub async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

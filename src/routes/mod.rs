pub mod greet;
pub mod health_check;
pub mod subscriptions;

pub async fn index() -> &'static str {
    "Hello, World!"
}

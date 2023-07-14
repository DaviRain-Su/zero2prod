mod greet;
mod health_check;
mod subscriptions;

pub use greet::*;
pub use health_check::*;
pub use subscriptions::*;

pub async fn index() -> &'static str {
    "Hello, World!"
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid subscriber name Error: {0}")]
    ValidateSubscriberNameError(String),
}

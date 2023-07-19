use axum::body::Body;
use axum::http::Response;
use axum::http::{request::Parts, StatusCode};
use axum::{
    async_trait,
    extract::{Form, FromRef, FromRequestParts, State},
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Acquire, PgConnection};

use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

// we can extract the connection pool with `State`
pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, conn_pool),
    fields(request_id = %Uuid::new_v4(),subscriber = ?form)
)]
pub async fn subscribe(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    form: Option<Form<FormData>>,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");
    match form {
        Some(form) => match insert_subscriber(connection_pool, &form).await {
            Ok(_) => {
                // if !is_valid_name(&form.0.name) {
                //     let error_text = "invalid form name";
                //     let mut resonse = Response::new(Body::from(error_text));
                //     *resonse.status_mut() = StatusCode::BAD_REQUEST;
                //     return resonse;
                // }
                let response_text = format!(
                    "Received subscription from {} at {}",
                    form.0.name, form.0.email
                );
                Response::new(Body::from(response_text))
            }
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                let error_text = format!("Database error: {}", e);
                let mut response = Response::new(Body::from(error_text));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response
            }
        },
        None => {
            let error_text = "Missing fields";
            let mut response = Response::new(Body::from(error_text));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response
        }
    }
    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &mut PgConnection,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

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
use sqlx::Acquire;
use tracing::Instrument;
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

// 这里我的疑问是，这里的两个参数的位置不能颠倒
// Let's start simple: we always return a 200 OK
pub async fn subscribe(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    form: Option<Form<FormData>>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.", %request_id,
        subscriber = ?form,
    );
    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    // Here you can use the form data.
    match form {
        Some(form) => {
            let connection = conn_pool.acquire().await.unwrap();
            let result = sqlx::query!(
                r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
                Uuid::new_v4(),
                form.email,
                form.name,
                Utc::now()
            )
            .execute(connection)
            // First we attach the instrumentation, then we `.await` it
            .instrument(query_span)
            .await;
            match result {
                Ok(_) => {
                    tracing::info!(
                        "request_id {} - New subscriber details have been saved",
                        request_id
                    );
                    let response_text = format!(
                        "Received subscription from {} at {}",
                        form.0.name, form.0.email
                    );
                    Response::new(Body::from(response_text))
                }
                Err(e) => {
                    tracing::error!(
                        "request_id {} - Failed to execute query: {:?}",
                        request_id,
                        e
                    );
                    let error_text = format!("Database error: {}", e);
                    let mut response = Response::new(Body::from(error_text));
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    response
                }
            }
        }
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

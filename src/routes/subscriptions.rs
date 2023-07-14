use axum::body::Body;
use axum::http::Response;
use axum::http::StatusCode;
use axum::{extract::Form, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(form: Option<Form<FormData>>) -> impl IntoResponse {
    // Here you can use the form data.
    match form {
        Some(form) => {
            let response_text = format!(
                "Received subscription from {} at {}",
                form.0.name, form.0.email
            );
            Response::new(Body::from(response_text))
        }
        None => {
            let error_text = "Missing fields";
            let mut response = Response::new(Body::from(error_text));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response
        }
    }
}

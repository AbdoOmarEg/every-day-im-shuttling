use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn root() -> impl IntoResponse {
    Html("Hello, Worldz!")
}

pub async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

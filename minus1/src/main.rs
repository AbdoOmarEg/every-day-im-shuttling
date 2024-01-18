use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app: Router = Router::new()
        .route("/", get(handler))
        .route("/-1/error", get(fake_error));
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn handler() -> impl IntoResponse {
    Html("Hello, Worldz!")
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

use axum::{
    routing::{get, post},
    serve, Router,
};
use day1::any_number_of_numbers;
use day4::{contest, strength};
use day5::optional_queryiz;
use minus1::{fake_error, root};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/-1/error", get(fake_error))
        .route("/1/*numberz", get(any_number_of_numbers))
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
        .route("/5", post(optional_queryiz));
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    serve(listener, app).await.unwrap()
}

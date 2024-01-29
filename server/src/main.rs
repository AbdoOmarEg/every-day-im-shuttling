use axum::{
    routing::{get, post},
    serve, Router,
};
use day1::any_number_of_numbers;
use day4::{contest, strength};
use day5::optional_queryiz;
use day6::find_elfs;
use day7::{bake, header_decode_recpie_cookie};
use day8::{drop_gravity, poki_weight};
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
        .route("/5", post(optional_queryiz))
        .route("/6", post(find_elfs))
        .route("/7/decode", get(header_decode_recpie_cookie))
        .route("/7/bake", get(bake))
        .route("/8/weight/:idx", get(poki_weight))
        .route("/8/drop/:idx", get(drop_gravity));
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    serve(listener, app).await.unwrap()
}

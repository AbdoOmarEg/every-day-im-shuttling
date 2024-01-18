use axum::{extract::Path, response::IntoResponse, routing::get, serve, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // num1 can be any variable name doesn't need to match the struct
    // let app = Router::new().route("/1/:numsez1/:numzes2", get(any_number_of_numbers));
    // numberz must be the same variable name in the struct
    let app = Router::new().route("/1/*numberz", get(any_number_of_numbers));
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

// #[derive(Deserialize)]
// struct Numbers {
//     num1: Option<i32>,
//     num2: Option<i32>,
// }
//
// async fn handler(Path(numbers): Path<Numbers>) -> impl IntoResponse {
//     let rez = (numbers.num1.unwrap() ^ numbers.num2.unwrap()).pow(3);
//     rez.to_string()
// }

// bonus
#[derive(Deserialize)]
struct NumbersString {
    numberz: String,
}

async fn any_number_of_numbers(Path(numbers_string): Path<NumbersString>) -> impl IntoResponse {
    let rez = numbers_string.numberz;
    println!("rez={:?}", rez);
    let rez: Vec<i32> = rez.split('/').map(|f| f.parse::<i32>().unwrap()).collect();

    let mut sumz = rez[0];
    for &num in rez.iter().skip(1) {
        println!("num={:?}", num);
        sumz ^= num;
    }

    let x = sumz.pow(3).to_string();
    x
}

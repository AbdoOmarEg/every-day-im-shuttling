use axum::{extract::Path, response::IntoResponse};
use serde::Deserialize;

// bonus
#[derive(Deserialize)]
pub struct NumbersString {
    numberz: String,
}

pub async fn any_number_of_numbers(Path(numbers_string): Path<NumbersString>) -> impl IntoResponse {
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

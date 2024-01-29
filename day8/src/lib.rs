use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use reqwest::Client;

#[derive(serde::Deserialize)]
pub struct PokeIdx {
    idx: usize,
}

#[derive(serde::Deserialize, Debug)]
pub struct PokeWeight {
    weight: usize,
}

pub async fn poki_weight(Path(poke_idx): Path<PokeIdx>) -> impl IntoResponse {
    let idx = poke_idx.idx;
    let client = Client::new();
    let res = client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", idx))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let weight = serde_json::from_str::<PokeWeight>(&res).unwrap().weight / 10;
    println!("weight={:#?}", weight);

    // println!("res={}, weight:{}", res, &res[weight..weight + 4]);

    Json(weight)
}

pub async fn drop_gravity(Path(poke_idx): Path<PokeIdx>) -> impl IntoResponse {
    let client = Client::new();
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", poke_idx.idx);
    let Ok(body) = client
        .get(url)
        .send()
        .await
        .expect("couldn't send a request")
        .text()
        .await
    else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error with Pokemon API").into_response();
    };

    let weight = serde_json::from_str::<PokeWeight>(&body)
        .expect("Couldn't Deserialize the json")
        .weight as f64
        / 10.0;

    const GRAVATATIONAL_CONSTANT_EARTH: f64 = 9.825;
    const HEIGHT: f64 = 10.0;
    let velocity = (2.0 * GRAVATATIONAL_CONSTANT_EARTH * HEIGHT).sqrt();
    let momentum = weight * velocity;

    (StatusCode::OK, Json(momentum)).into_response()
}

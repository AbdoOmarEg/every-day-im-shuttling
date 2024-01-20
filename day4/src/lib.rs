use axum::{extract, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Reindeer {
    #[allow(dead_code)]
    name: String,
    strength: i32,
}

pub async fn strength(extract::Json(reindeers): extract::Json<Vec<Reindeer>>) -> impl IntoResponse {
    reindeers
        .iter()
        .map(|f| f.strength)
        .sum::<i32>()
        .to_string()
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ReindeersContest {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    strength: i32,
    speed: f32,
    height: i32,
    #[allow(dead_code)]
    antler_width: i32,
    snow_magic_power: i32,
    #[allow(dead_code)]
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    cAnD13s_3ATeN_yesT3rdAy: i32,
}

// can send this as json
// #[derive(Clone, Deserialize, Debug, Serialize)]
// pub struct ContestResult {
//     pub fastest: String,
//     pub tallest: String,
//     pub magician: String,
//     pub consumer: String,
// }

pub async fn contest(
    extract::Json(reindeers_contest): extract::Json<Vec<ReindeersContest>>,
) -> impl IntoResponse {
    // boilerplate but one loop instead of iterators
    let mut fastest = reindeers_contest[0].speed;
    let mut tallest = reindeers_contest[0].height;
    let mut magician = reindeers_contest[0].snow_magic_power;
    let mut consumer = reindeers_contest[0].cAnD13s_3ATeN_yesT3rdAy;
    let mut fastest_name = reindeers_contest[0].name.to_string();
    let mut tallest_name = reindeers_contest[0].name.to_string();
    let mut magician_name = reindeers_contest[0].name.to_string();
    let mut consumer_name = reindeers_contest[0].name.to_string();
    for reindeer in reindeers_contest.iter().skip(1) {
        if reindeer.speed > fastest {
            fastest = fastest.max(reindeer.speed);
            fastest_name = reindeer.name.to_string();
        }
        if reindeer.height > tallest {
            tallest = tallest.max(reindeer.height);
            tallest_name = reindeer.name.to_string();
        }
        if reindeer.snow_magic_power > magician {
            magician = magician.max(reindeer.snow_magic_power);
            magician_name = reindeer.name.to_string();
        }
        if reindeer.cAnD13s_3ATeN_yesT3rdAy > consumer {
            consumer = consumer.max(reindeer.cAnD13s_3ATeN_yesT3rdAy);
            consumer_name = reindeer.name.to_string();
        }
    }

    Json(json!( {
        "fastest": format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest, fastest_name
        ),
        "tallest": format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest_name, tallest
        ),
        "magician": format!(
            "{} could blast you away with a snow magic power of {}",
            magician_name, magician
        ),
        "consumer": format!(
            "{} ate lots of candies, but also some {}",
            consumer_name, consumer
        ),
    }))
}

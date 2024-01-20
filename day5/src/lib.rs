use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Names {
    name: String,
}

// basic task
// pub async fn queryiz(
//     Query(pagination): Query<Pagination>,
//     Json(names): Json<Vec<String>>,
// ) -> impl IntoResponse {
//     println!("{:?}", pagination);
//     let x =
//         &names[pagination.offset as usize..pagination.offset as usize + pagination.limit as usize];
//     Json(x).into_response()
// }

// bonus
pub async fn optional_queryiz(
    Query(pagination): Query<Pagination>,
    Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
    println!("{:?}", pagination);
    let offset = pagination.offset.unwrap_or(0);
    let limit = pagination.limit;
    let split = pagination.split;

    let names = match limit {
        Some(l) => &names[offset..offset + l],
        None => &names[offset..names.len()],
    };

    let mut response = vec![];

    // using mod  but mod is usually slower
    let mut start_split = 0;
    if let Some(s) = split {
        // to avoid dividing by zero
        if s > 0 {
            for end_split in 1..names.len() {
                if end_split % s == 0 {
                    response.push(&names[start_split..end_split]);
                    start_split = end_split;
                }
            }
        }
    }

    // without mod
    // let mut start_split = 0;
    // let mut end_split = 0;
    // for end in 0..names.len() {
    //     end_split += 1;
    //     if end_split == split {
    //         let x = &names[start_split..=end];
    //         println!("x={:?}", x);
    //         vec.push(x);
    //         start_split = end + 1;
    //         end_split = 0;
    //     }
    // }

    // push last split if there is any
    if start_split != names.len() {
        response.push(&names[start_split..names.len()]);
    }

    Json(response).into_response()
}

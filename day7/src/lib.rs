use std::collections::HashMap;

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    flour: i32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Ingredients {
    flour: u64,
    sugar: u64,
    butter: u64,
    #[serde(rename = "baking powder")]
    baking_powder: u64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u64,
}

// impl Ingredients {
//     fn iter(&self) -> IngredientsIterator {
//         IngredientsIterator {
//             ingredients: self,
//             index: 0,
//         }
//     }
// }
//
// struct IngredientsIterator<'a> {
//     ingredients: &'a Ingredients,
//     index: u64,
// }
//
// impl<'a> Iterator for IngredientsIterator<'a> {
//     type Item = (String, u64);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let field_name = match self.index {
//             0 => String::from("flour"),
//             1 => String::from("sugar"),
//             2 => String::from("butter"),
//             3 => String::from("baking powder"),
//             4 => String::from("chocolate chips"),
//             _ => return None,
//         };
//
//         let field_value = match self.index {
//             0 => self.ingredients.flour,
//             1 => self.ingredients.sugar,
//             2 => self.ingredients.butter,
//             3 => self.ingredients.baking_powder,
//             4 => self.ingredients.chocolate_chips,
//             _ => return None,
//         };
//
//         self.index += 1;
//
//         Some((field_name, field_value))
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
struct _Raceeba {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Serialize, Deserialize, Debug)]
struct CookiesAndPantryLeft {
    cookies: usize,
    pantry: Ingredients,
}

#[derive(Serialize, Deserialize, Debug)]
struct Raceeba {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

// task 1
pub async fn header_decode_recpie_cookie(header: HeaderMap) -> impl IntoResponse {
    println!("header={:?}", header.get("cookie"));

    if let Some(cookie) = header.get("cookie") {
        println!("cookie={:?}", cookie);

        if let Ok(recipe_str) = cookie.to_str() {
            println!("recipe_str={:?}", recipe_str);

            let recipe_value = recipe_str.splitn(2, '=').nth(1).unwrap();
            println!("recipe_value={:?}", recipe_value);

            if let Ok(decoded) = BASE64_STANDARD.decode(recipe_value) {
                println!("decoded={:?}", decoded);

                if let Ok(recipe_decoded) = String::from_utf8(decoded) {
                    println!("recipe_decoded={}", recipe_decoded);

                    if let Ok(parsed_json) = serde_json::from_str::<Recipe>(&recipe_decoded) {
                        println!("parsed_json={:?}", parsed_json);
                        return Json(parsed_json);
                    } else {
                        println!("Error parsing JSON");
                    }
                }
            }
        }
    }

    // Return a generic response if decoding or parsing fails
    unreachable!()
}

// task 2
pub async fn _bake(header: HeaderMap) -> impl IntoResponse {
    println!("header={:?}\n", header);
    if let Some(cookie) = header.get("cookie") {
        println!("cookie={:?}", cookie);

        if let Ok(recipe_str) = cookie.to_str() {
            println!("recipe_str={:?}", recipe_str);

            let recipe_value = recipe_str.splitn(2, "=").nth(1).unwrap();

            if let Ok(decoded_bytes) = BASE64_STANDARD.decode(recipe_value) {
                println!("decoded={:?}", decoded_bytes);

                if let Ok(decoded_string) = String::from_utf8(decoded_bytes) {
                    println!("decoded_string={:?}", decoded_string);

                    if let Ok(parsed_json) = serde_json::from_str::<_Raceeba>(&decoded_string) {
                        println!("parsed_json={:?}", parsed_json);

                        let recipe = parsed_json.recipe;
                        let mut pantry = parsed_json.pantry.clone();

                        let mut count_flour = 0;
                        let mut count_sugar = 0;
                        let mut count_butter = 0;
                        let mut count_baking_powder = 0;
                        let mut count_chocolate_chips = 0;

                        while pantry.flour >= recipe.flour {
                            pantry.flour -= recipe.flour;
                            count_flour += 1;
                        }
                        while pantry.sugar >= recipe.sugar {
                            pantry.sugar -= recipe.sugar;
                            count_sugar += 1;
                        }
                        while pantry.butter >= recipe.butter {
                            pantry.butter -= recipe.butter;
                            count_butter += 1;
                        }
                        while pantry.baking_powder >= recipe.baking_powder {
                            pantry.baking_powder -= recipe.baking_powder;
                            count_baking_powder += 1;
                        }
                        while pantry.chocolate_chips >= recipe.chocolate_chips {
                            pantry.chocolate_chips -= recipe.chocolate_chips;
                            count_chocolate_chips += 1;
                        }

                        let cookies = count_flour
                            .min(count_sugar)
                            .min(count_butter)
                            .min(count_baking_powder)
                            .min(count_chocolate_chips);

                        let mut new_pantry = parsed_json.pantry;

                        for _ in 0..cookies {
                            new_pantry.flour -= recipe.flour;
                            new_pantry.sugar -= recipe.sugar;
                            new_pantry.butter -= recipe.butter;
                            new_pantry.baking_powder -= recipe.baking_powder;
                            new_pantry.chocolate_chips -= recipe.chocolate_chips;
                        }

                        let cookies_and_pantry_left = CookiesAndPantryLeft {
                            cookies,
                            pantry: new_pantry,
                        };

                        return Json(cookies_and_pantry_left).into_response();
                    }
                }
            }
        }
    }

    unreachable!()
}

#[derive(Serialize)]
struct InnovativeRecipe {
    cookies: usize,
    pantry: HashMap<String, u64>,
}

// bonus
pub async fn bake(header: HeaderMap) -> impl IntoResponse {
    println!("header={:?}\n", header);
    if let Some(cookie) = header.get("cookie") {
        println!("cookie={:?}", cookie);

        if let Ok(recipe_str) = cookie.to_str() {
            println!("recipe_str={:?}", recipe_str);

            let recipe_value = recipe_str.splitn(2, "=").nth(1).unwrap();

            if let Ok(decoded_bytes) = BASE64_STANDARD.decode(recipe_value) {
                println!("decoded={:?}", decoded_bytes);

                if let Ok(decoded_string) = String::from_utf8(decoded_bytes) {
                    println!("decoded_string={:?}", decoded_string);

                    if let Ok(parsed_json) = serde_json::from_str::<Raceeba>(&decoded_string) {
                        println!("parsed_json={:?}", parsed_json);

                        let recipe = parsed_json.recipe;
                        let pantry = parsed_json.pantry;

                        let mut cookies_num: Option<u64> = None;

                        for (recipe_ingredient, recipe_amount) in recipe.iter() {
                            let pantry_amount = pantry.get(recipe_ingredient).unwrap_or(&0);
                            if pantry_amount != &0 {
                                if pantry_amount >= recipe_amount {
                                    if let Some(c) = cookies_num {
                                        cookies_num = Some(c.min(pantry_amount / recipe_amount));
                                    } else {
                                        cookies_num = Some(pantry_amount / recipe_amount);
                                    }
                                }
                            }
                        }

                        println!("cookies_num={:#?}", cookies_num);

                        let mut new_pantry: HashMap<String, u64> = HashMap::new();
                        for (pantry_ingredient, pantry_amount) in &pantry {
                            let recipe_amount = recipe.get(pantry_ingredient).unwrap_or(&0);
                            println!("hello");
                            println!("pantry_amount={:#?}", pantry_amount);
                            println!("recipe_amount={:#?}", recipe_amount);
                            println!("cookies_num={:#?}", cookies_num);
                            new_pantry.insert(
                                pantry_ingredient.clone(),
                                pantry_amount - (*recipe_amount * cookies_num.unwrap_or(0)),
                            );
                        }

                        println!("cookies_num={:#?}", cookies_num);

                        let innovative_recipe = InnovativeRecipe {
                            cookies: cookies_num.unwrap_or(0) as usize,
                            pantry: new_pantry,
                        };

                        return Json(innovative_recipe).into_response();
                    }
                }
            }
        }
    }

    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
}

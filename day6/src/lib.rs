use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct ElfCounter {
    elf: usize,
}

pub async fn _find_elfs(text: String) -> impl IntoResponse {
    let elf_count = text.matches("elf").count();
    println!("count={:?}", elf_count);

    Json(ElfCounter { elf: elf_count })
}

#[derive(Serialize)]
struct ElfShelfNoShelfCounter {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

pub async fn find_elfs(text: String) -> impl IntoResponse {
    let mut elf = 0;
    let mut elf_on_a_shelf = 0;
    let mut shelf_with_no_elf_on_it = 0;

    // this way is faster than doing iterators on the first two then for loop on shelf with no elf
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 0..words.len() {
        let word = words[i];
        // elf
        if word.len() >= 3 {
            for j in 0..=word.len() - 3 {
                if &word[j..j + 3] == "elf" {
                    elf += 1;

                    // elf on a shelf
                    if i + 3 < words.len()
                        && words[i + 1] == "on"
                        && words[i + 2] == "a"
                        && words[i + 3] == "shelf"
                    {
                        elf_on_a_shelf += 1;
                    }
                }
            }
        }

        // shelf with no elf
        if word.len() >= 5 {
            if word == "shelf" {
                println!("i={:?}", i);
                if i as i32 - 3 >= 0 {
                    if words[i - 1] != "a" || words[i - 2] != "on" || words[i - 3] != "elf" {
                        shelf_with_no_elf_on_it += 1;
                    }
                } else {
                    shelf_with_no_elf_on_it += 1;
                }
            }
        }
    }

    Json(ElfShelfNoShelfCounter {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}

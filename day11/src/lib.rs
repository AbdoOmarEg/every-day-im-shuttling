use axum::extract::Multipart;
use axum::{http::HeaderMap, response::IntoResponse};

const DECORATION_IMAGE: &[u8] = include_bytes!("../../assets/decoration.png");

pub async fn image() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "image/png".parse().unwrap());
    (headers, DECORATION_IMAGE).into_response()
}

pub async fn red_pixels(mut multipart: Multipart) -> impl IntoResponse {
    let mut num_magic_pixels = 0;
    println!("hellozz");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        if field_name == "image" {
            let data = field.bytes().await.unwrap();
            let image = image::load_from_memory(&data[..]).unwrap().into_rgb8();

            for pixel in image.pixels() {
                let rgb = pixel.0;
                println!("rgb={:?}", rgb);
                let red = rgb[0] as u32;
                let green = rgb[1] as u32;
                let blue = rgb[2] as u32;
                if red > blue + green {
                    num_magic_pixels += 1;
                }
            }
        } else {
            let data = field.text().await.unwrap();
            println!("field_name={:?}, data={:?}", field_name, data);
        }
    }
    num_magic_pixels.to_string()
}

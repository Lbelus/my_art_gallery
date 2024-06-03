use askama::Template;
use serde::{Deserialize, Serialize};
use warp::Filter;
use dotenv::dotenv;
use std::env;
use std::fs;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "img_template.html")]
struct ImgTemplate<'a> {
    alt: &'a str,
    src: &'a str,
    title_en: &'a str,
    title_fr: &'a str,
    description_en: &'a str,
    description_fr: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
struct ImageData {
    id: String,
    alt: String,
    src: String,
    title_en: String,
    title_fr: String,
    description_en: String,
    description_fr: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap();

    let data = fs::read_to_string("static/images.json").expect("Unable to read file");
    let images: Vec<ImageData> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    let template_route = warp::path::end()
        .map(|| {
            let template = IndexTemplate;
            warp::reply::html(template.render().unwrap())
        });

    let img_route = warp::path!("img" / String)
        .map(move |id: String| {
            let img_data = images.iter().find(|&img| img.id == id).cloned();
            if let Some(data) = img_data {
                let template = ImgTemplate {
                    alt: &data.alt,
                    src: &data.src,
                    title_en: &data.title_en,
                    title_fr: &data.title_fr,
                    description_en: &data.description_en,
                    description_fr: &data.description_fr,
                };
                warp::reply::html(template.render().unwrap())
            } else {
                warp::reply::html("Image not found".to_string())
            }
        });

    let static_files = warp::fs::dir("static");

    let routes = template_route.or(img_route).or(static_files);

    log::info!("Starting server at port {}", port);
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

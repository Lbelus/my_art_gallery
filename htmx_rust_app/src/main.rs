use askama::Template;
use warp::Filter;
use dotenv::dotenv;
use std::env;


#[derive(Template)]
#[template(path = "example.html")]
struct ExampleTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
   dotenv().ok();
    env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap(); 
    let template_route = warp::path("example")
        .map(|| {
            let template = ExampleTemplate { name: "HTMX + Rust".to_string() };
            warp::reply::html(template.render().unwrap())
        });
    let static_files = warp::fs::dir("static");
    let hello = warp::path("hello")
        .map(|| warp::reply::html("Hello, HTMX + Rust!"));
    let routes = template_route.or(hello).or(static_files);
    
    log::info!("Starting server at port {}", port);
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}


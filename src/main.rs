use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub mod models;
pub mod media;
pub mod markdown;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::home::home))
        .route("/projects", get(routes::projects::projects))
        .route("/articles", get(routes::articles::articles))
        .route("/articles/{slug}", get(routes::articles::article))
        .route("/gallery", get(routes::gallery::gallery))
        .route("/gallery/{slug}", get(routes::gallery::album))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/media", ServeDir::new("media"));

    let listener = tokio::net::TcpListener::bind("[::]:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

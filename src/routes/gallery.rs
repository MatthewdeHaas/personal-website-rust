use axum::{
    extract::Path,
    response::Html
};
use askama::Template;
use crate::models::Album;
use crate::media::ALBUMS;

#[derive(Template)]
#[template(path = "pages/gallery.html")]
struct GalleryTemplate {
    current_path: &'static str,
    albums: Vec<&'static Album>,
}

#[derive(Template)]
#[template(path = "pages/album.html")]
struct AlbumTemplate<'a> {
    current_path: String,
    album: &'a Album,
}

pub async fn gallery() -> Html<String> {
    let template = GalleryTemplate {
        current_path: "/gallery",
        albums: ALBUMS.values().collect(),
    };
    Html(template.render().unwrap())
}

pub async fn album(Path(slug): Path<String>) -> Html<String> {
    let template = AlbumTemplate {
        current_path: format!("/gallery/{}", slug),
        album: ALBUMS.get(&slug).unwrap(),
    };
    Html(template.render().unwrap())
}

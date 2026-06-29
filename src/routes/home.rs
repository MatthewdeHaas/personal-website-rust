use axum::response::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {
    current_path: &'static str,
}

pub async fn home() -> Html<String> {
    let template = HomeTemplate {
        current_path: "/"
    };
    Html(template.render().unwrap())
}

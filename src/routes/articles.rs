use axum::{
    extract::Path,
    response::Html
};
use askama::Template;
use crate::models::Post;
use crate::markdown::POSTS;


#[derive(Template)]
#[template(path = "pages/articles.html")]
struct ArticlesTemplate {
    current_path: &'static str,
    posts: &'static Vec<Post>,
}

#[derive(Template)]
#[template(path = "pages/article.html")]
struct ArticleTemplate<'a> {
    current_path: String,
    post: &'a Post,
    content: String,
}

pub async fn articles() -> Html<String> {
    let template = ArticlesTemplate {
        current_path: "/articles",
        posts: &*POSTS,
    };
    Html(template.render().unwrap())
}

pub async fn article(Path(slug): Path<String>) -> Html<String> {
    let post = POSTS.iter().find(|p| p.slug == slug).unwrap();
    let template = ArticleTemplate {
        current_path: format!("/articles/{}", slug),
        content: post.content.clone(),
        post,
    };
    Html(template.render().unwrap())
}

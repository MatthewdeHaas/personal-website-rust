use pulldown_cmark::{Parser, html};
use crate::models::Post;
use std::sync::LazyLock;

pub static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| get_posts());

pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn get_posts() -> Vec<Post> {
    let dir = std::fs::read_dir("content/posts").unwrap();
    let mut posts = Vec::new();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        let raw = std::fs::read_to_string(&path).unwrap();

        let trimmed = raw.strip_prefix("---").unwrap_or(&raw);
        let (frontmatter, body) = trimmed.split_once("---").unwrap_or(("", &raw));

        let mut title = String::new();
        let mut date = String::new();
        let mut description = String::new();

        for line in frontmatter.lines() {
            if let Some(val) = line.strip_prefix("title:") {
                title = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("date:") {
                date = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("description:") {
                description = val.trim().to_string();
            }
        }

        let content = render_markdown(body.trim());

        posts.push(Post { slug, title, date, description, content });
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

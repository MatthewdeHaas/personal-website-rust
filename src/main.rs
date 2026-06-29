use axum::{
    routing::get,
    Router, 
    extract::Path, 
    response::Html
};
use askama::Template;
use pulldown_cmark::{Parser, html};
use tower_http::services::ServeDir;
use std::sync::LazyLock;
use serde::Deserialize;
use chrono::NaiveDate;
use std::collections::HashMap;


#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    current_path: &'static str,
}

async fn index() -> Html<String> {
    let template = IndexTemplate {
        current_path: "/"
    };
    Html(template.render().unwrap())
}


struct ProjectLink {
    label: &'static str,
    href: &'static str,
}

struct Project {
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
    year: &'static str,
    image: Option<&'static str>,
    links: Vec<ProjectLink>,
}

#[derive(Template)]
#[template(path = "pages/projects.html")]
struct ProjectsTemplate {
    current_path: &'static str,
    projects: &'static Vec<Project>,
}


static PROJECTS: LazyLock<Vec<Project>> = LazyLock::new(|| {
    vec![
        Project {
            title: "Gambit",
            description: "A peer-to-peer betting platform built on Polygon...",
            tags: vec!["SvelteKit", "Solidity", "Postgres", "Ethers.js", "IPFS"],
            year: "2025",
            image: None,
            links: vec![
                ProjectLink { label: "GitHub", href: "https://github.com/MatthewdeHaas/gambit-svelte" },
            ],
        },
        Project {
            title: "Personal Site",
            description: "This site. Built with Rust and Alpine",
            tags: vec!["Rust", "Alpine.js"],
            year: "2026",
            image: None,
            links: vec![
                ProjectLink { label: "GitHub", href: "https://github.com/yourname/personal-site" },
            ],
        },
    ]
});

async fn projects() -> Html<String> {
    let template = ProjectsTemplate {
        current_path: "/projects",
        projects: &PROJECTS,
    };
    Html(template.render().unwrap())
}

struct Post {
    slug: String,
    title: String,
    date: String,
    description: String,
    content: String,
}

fn render_markdown(input: &str) -> String {
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


#[derive(Template)]
#[template(path = "pages/articles.html")]
struct ArticlesTemplate {
    current_path: &'static str,
    posts: Vec<Post>,
}

async fn articles() -> Html<String> {
    let template = ArticlesTemplate {
        current_path: "/articles",
        posts: get_posts(),
    };
    Html(template.render().unwrap())
}


#[derive(Template)]
#[template(path = "pages/article.html")]
struct ArticleTemplate {
    current_path: String,
    post: Post,
    content: String,

}
async fn article(Path(slug): Path<String>) -> Html<String> {
    let post = get_posts().into_iter().find(|p| p.slug == slug).unwrap();
    let template = ArticleTemplate {
        current_path: format!("/articles/{slug}"),
        content: post.content.clone(),
        post,
    };
    Html(template.render().unwrap())
}


struct Asset {
    file_name: String,
    asset_type: String,
}

struct Album {
    title: String,
    slug: String,
    description: String,
    date: NaiveDate,
    cover_url: String,
    assets: Vec<Asset>
}

// For metadata.json validation
#[derive(Deserialize)]
struct AlbumMetadata {
    description: String,
    date: NaiveDate,
    cover_url: Option<String>
}


#[derive(Template)]
#[template(path = "pages/gallery.html")]
struct GalleryTemplate {
    current_path: &'static str,
    albums: Vec<&'static Album>,
}

fn scan_albums() -> Vec<Album> {

    let album_dirs = std::fs::read_dir("media/albums").unwrap();
    let mut albums: Vec<Album> = Vec::new();

    // Loop through all albums
    for album_dir in album_dirs {
        // Read the json as a raw string
        let album_dir = album_dir.unwrap();
        let metadata_string = std::fs::read_to_string(album_dir.path().join("metadata.json")).unwrap();

        // parse the string as json; comparing it to the metadata schema defined
        let metadata: AlbumMetadata = serde_json::from_str(&metadata_string)
            .expect(&format!("invalid metadata.json in {:?}", album_dir.path()));
  
        // Define the contents of each album
        let asset_files = std::fs::read_dir(album_dir.path()).unwrap();
        let mut assets: Vec<Asset> = Vec::new();

        // Loop over the sub-directory
        for asset_file in asset_files { 
            // Validate extensions
            let asset_file = asset_file.unwrap();
            let path = asset_file.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == "metadata.json" {
                continue;
            }
            if !ALLOWED_ASSET_TYPES.contains(&ext) {
                continue;
            }

            let asset_type = if VIDEO_EXTENSIONS.contains(&ext) {
                "video".to_string()
            } else {
                "photo".to_string()
            };

            // Push the asset
            let asset = Asset {
                file_name: file_name.to_string(),
                asset_type,
            };
            assets.push(asset);
        }

        // Push the album
        let slug = album_dir.file_name().into_string().unwrap();

        // convert from kebab-case to Title Case
        let title = slug.replace('-', " ")
            .split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        let album = Album {
            title,
            slug,
            description: metadata.description,
            date: metadata.date,
            cover_url: metadata.cover_url.unwrap_or_else(|| "".to_string()),
            assets,
        };
        albums.push(album);
    }

    albums
}

static ALBUMS: LazyLock<HashMap<String, Album>> = LazyLock::new(|| {
    scan_albums().into_iter().map(|a| (a.slug.clone(), a)).collect()
});

const ALLOWED_ASSET_TYPES: &[&str] = &["jpg", "jpeg", "png", "heic", "webp", "mp4", "mov", "webm"];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "webm"];

async fn gallery() -> Html<String> {
    let template = GalleryTemplate {
        current_path: "/gallery",
        albums: ALBUMS.values().collect(),
    };
    Html(template.render().unwrap())
}


#[derive(Template)]
#[template(path = "pages/album.html")]
struct AlbumTemplate<'a> {
    current_path: String,
    album: &'a Album,
}

async fn album(Path(slug): Path<String>) -> Html<String> {
    let template = AlbumTemplate {
        current_path: format!("/gallery/{slug}"),
        album: ALBUMS.get(&slug).unwrap(),
    };
    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/projects", get(projects))
        .route("/articles", get(articles))
        .route("/articles/{slug}", get(article))
        .route("/gallery", get(gallery))
        .route("/gallery/{slug}", get(album))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("[::]:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

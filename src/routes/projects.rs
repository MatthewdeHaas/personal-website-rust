use axum::response::Html;
use askama::Template;
use std::sync::LazyLock;
use crate::models::{Project, ProjectLink};


pub static PROJECTS: LazyLock<Vec<Project>> = LazyLock::new(|| {
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

#[derive(Template)]
#[template(path = "pages/projects.html")]
struct ProjectsTemplate {
    current_path: &'static str,
    projects: &'static Vec<Project>,
}

pub async fn projects() -> Html<String> {
    let template = ProjectsTemplate {
        current_path: "/projects",
        projects: &PROJECTS,
    };
    Html(template.render().unwrap())
}

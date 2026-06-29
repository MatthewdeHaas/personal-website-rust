use serde::Deserialize;
use chrono::NaiveDate;

pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub content: String,
}

pub struct ProjectLink {
    pub label: &'static str,
    pub href: &'static str,
}

pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
    pub year: &'static str,
    pub image: Option<&'static str>,
    pub links: Vec<ProjectLink>,
}

pub struct Asset {
    pub file_name: String,
    pub asset_type: String,
}

pub struct Album {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub date: NaiveDate,
    pub cover_url: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub struct AlbumMetadata {
    pub description: String,
    pub date: NaiveDate,
    pub cover_url: Option<String>,
}

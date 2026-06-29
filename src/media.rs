use std::sync::LazyLock;
use std::collections::HashMap;
use crate::models::{Album, Asset, AlbumMetadata};


fn scan_albums() -> Vec<Album> {
    let album_dirs = std::fs::read_dir("media/albums").unwrap();
    let mut albums: Vec<Album> = Vec::new();

    for album_dir in album_dirs {
        let album_dir = album_dir.unwrap();
        let metadata_string = std::fs::read_to_string(album_dir.path().join("metadata.json")).unwrap();

        let metadata: AlbumMetadata = serde_json::from_str(&metadata_string)
            .expect(&format!("invalid metadata.json in {:?}", album_dir.path()));

        let asset_files = std::fs::read_dir(album_dir.path()).unwrap();
        let mut assets: Vec<Asset> = Vec::new();

        for asset_file in asset_files {
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

            let asset = Asset {
                file_name: file_name.to_string(),
                asset_type,
            };
            assets.push(asset);
        }

        let slug = album_dir.file_name().into_string().unwrap();
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

const ALLOWED_ASSET_TYPES: &[&str] = &["jpg", "jpeg", "png", "heic", "webp", "mp4", "mov", "webm"];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "webm"];

pub static ALBUMS: LazyLock<HashMap<String, Album>> = LazyLock::new(|| {
    scan_albums().into_iter().map(|a| (a.slug.clone(), a)).collect()
});

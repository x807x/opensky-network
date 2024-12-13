pub mod aircrafts;

use std::path::PathBuf;

use directories::ProjectDirs;
use log::debug;
use tokio::fs;

use crate::errors::Error;

const APPLICATION_NAME: &str = "opensky_api.rs";

async fn try_create_app_dir() {
    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(cache_dir).await;
}

fn get_cache_dir() -> PathBuf {
    let project_dir =
        ProjectDirs::from("", "", APPLICATION_NAME).expect("Failed to get base directory");
    project_dir.cache_dir().to_path_buf()
}
fn get_cache_path(filename: &str) -> PathBuf {
    let cache_dir = get_cache_dir();
    cache_dir.join(filename)
}

async fn load_from_cache(filename: &str) -> Result<Option<Vec<u8>>, Error> {
    let file_path = get_cache_path(filename);

    Ok(if file_path.exists() {
        debug!("File {} exists.", file_path.to_string_lossy());
        let file = fs::read(file_path).await?;
        Some(file)
    } else {
        None
    })
}

async fn download_to_cache(filename: &str, url: &str) -> Result<Vec<u8>, Error> {
    let file_path = get_cache_path(filename);

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    debug!("Write to file: {:?}", file_path);
    try_create_app_dir().await;
    fs::write(file_path, &bytes).await?;

    Ok(bytes.to_vec())
}

/*
async fn download_with_progress(filename: &str, url: &str, progress_bar: ProgressBar) -> Result<Vec<u8>, Error> {
    let file_path = get_cache_path(filename);

    let response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or_default();
    let mut stream = response.bytes_stream();

    let mut downloaded = 0;
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded = min(total_size, downloaded + chunk.len() as u64);
        progress_bar.set_position(downloaded);
        buffer.extend_from_slice(&chunk);
    }
    progress_bar.finish_with_message(format!("Downloaded: {}", filename));
    fs::write(file_path, &buffer).await?;

    Ok(buffer)
}*/

pub async fn load_data(filename: &str, url: &str) -> Result<Vec<u8>, Error> {
    match load_from_cache(filename).await? {
        Some(data) => Ok(data),
        None => download_to_cache(filename, url).await,
    }
}

pub async fn load_data_normalize(filename: &str, url: &str) -> Result<String, Error> {
    Ok(String::from_utf8_lossy(&load_data(filename, url).await?)
        .replace("\"", "\\\"")
        .replace("\'", "\""))
}

pub async fn clear_cache() {
    let project_dir =
        ProjectDirs::from("", "", APPLICATION_NAME).expect("Failed to get base directory");
    let cache_dir = project_dir.cache_dir();

    fs::remove_dir_all(cache_dir)
        .await
        .expect("Failed to remove cache");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, Window};
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DownloadProgress {
    id: String,
    downloaded: u64,
    total: u64,
    speed: f64,
    percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DownloadTask {
    id: String,
    url: String,
    filename: String,
    filepath: String,
    status: String, // "downloading", "paused", "completed", "error"
}

struct AppState {
    downloads: Arc<Mutex<Vec<DownloadTask>>>,
}

// Start a download
#[tauri::command]
async fn start_download(
    url: String,
    filename: String,
    save_path: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let download_id = uuid::Uuid::new_v4().to_string();
    
    // Add to downloads list
    let task = DownloadTask {
        id: download_id.clone(),
        url: url.clone(),
        filename: filename.clone(),
        filepath: save_path.clone(),
        status: "downloading".to_string(),
    };
    
    {
        let mut downloads = state.downloads.lock().unwrap();
        downloads.push(task);
    }

    // Spawn download task
    tokio::spawn(async move {
        if let Err(e) = download_file(&url, &save_path, &download_id, window.clone()).await {
            let _ = window.emit("download-error", format!("Download failed: {}", e));
        }
    });

    Ok(download_id)
}

async fn download_file(
    url: &str,
    save_path: &str,
    download_id: &str,
    window: Window,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    
    let mut file = File::create(save_path)?;
    let mut last_update = std::time::Instant::now();
    let mut last_downloaded = 0u64;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        
        downloaded += chunk.len() as u64;
        
        // Update progress every 200ms
        if last_update.elapsed().as_millis() > 200 {
            let speed = ((downloaded - last_downloaded) as f64 
                / last_update.elapsed().as_secs_f64()) / 1024.0; // KB/s
            
            let progress = DownloadProgress {
                id: download_id.to_string(),
                downloaded,
                total: total_size,
                speed,
                percentage: if total_size > 0 {
                    (downloaded as f64 / total_size as f64) * 100.0
                } else {
                    0.0
                },
            };
            
            let _ = window.emit("download-progress", progress);
            
            last_update = std::time::Instant::now();
            last_downloaded = downloaded;
        }
    }

    // Emit completion
    let _ = window.emit("download-complete", download_id);
    
    Ok(())
}

// Get all downloads
#[tauri::command]
fn get_downloads(state: State<'_, AppState>) -> Vec<DownloadTask> {
    let downloads = state.downloads.lock().unwrap();
    downloads.clone()
}

// Remove download
#[tauri::command]
fn remove_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut downloads = state.downloads.lock().unwrap();
    downloads.retain(|d| d.id != id);
    Ok(())
}

// Open download folder
#[tauri::command]
async fn open_download_folder() -> Result<(), String> {
    let download_dir = tauri::api::path::download_dir()
        .ok_or("Could not find download directory")?;
    
    tauri::api::shell::open(&tauri::api::shell::Scope::Global, download_dir.to_str().unwrap(), None)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            downloads: Arc::new(Mutex::new(Vec::new())),
        })
        .invoke_handler(tauri::generate_handler![
            start_download,
            get_downloads,
            remove_download,
            open_download_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
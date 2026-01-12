// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod apple_music;
mod art_server;
mod discord;

use discord::DiscordClient;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use std::fs;
use tauri::{menu::{Menu, MenuItem}, tray::{TrayIconBuilder}};

#[tauri::command]
fn get_current_track() -> Option<String> {
    apple_music::get_track_info()
}

fn upload_album_art(file_path: &str, worker_upload_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = fs::read(file_path)?;

    let client = Client::new();

    let resp = client.post(worker_upload_url)
        .header("Content-Type", "image/png")
        .body(bytes)
        .send()?;

    if !resp.status().is_success() {
        return Err(format!("Upload failed: {}", resp.status()).into());
    }

    #[derive(serde::Deserialize)]
    struct UploadResponse {
        url: String,
    }

    let json: UploadResponse = resp.json()?;

    Ok(json.url)
}

fn main() {
    println!("===========================================================");
    println!("Apple Music Discord Rich Presence");
    println!("made with much love by Squishy :) <3");
    println!("===========================================================");
    art_server::start_art_server();

    let discord = Arc::new(DiscordClient::new());
    let presence_enabled = Arc::new(AtomicBool::new(true));
    println!("\nMAIN:::Initialized Discord client.");
    {
        let discord = discord.clone();
        let presence_enabled = presence_enabled.clone();

        thread::spawn(move || {
            println!("THREAD:::Started Discord presence thread.");
            let mut last_track = String::new();
            let mut last_state = String::new();
            let mut last_position = f64::MIN;

            loop {
                println!("-----------------------------------------------------------");
                println!("THREAD:::Loop iteration started\n");
                if !presence_enabled.load(Ordering::Relaxed) {
                    discord.clear();
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }

                if let Some(info) = apple_music::get_track_info() {
                    println!("THREAD:::Fetched track info: {}", info);
                    let current_track = if info != "NO_TRACK" { info.clone().split("||").collect::<Vec<&str>>()[1].to_string() } else { String::new() };
                    if info != "NO_TRACK" {
                        let parts: Vec<&str> = info.split("||").collect();
                        if parts.len() >= 7 {
                            let state = parts[0];
                            let title = parts[1];
                            let artist = parts[2];
                            let album = parts[3];
                            let duration: f64 = parts[4].parse().unwrap_or(0.0);
                            let position: f64 = parts[5].parse().unwrap_or(0.0);
                            let art_path = parts[6];
                            let public_url = match upload_album_art(art_path, "https://apple-music-artwork.squishyapplemusicrpc.workers.dev/upload") {
                                Ok(result) => result,
                                Err(e) => {
                                    println!("ERROR:::Failed to upload album art: {}", e);
                                    String::new()
                                }
                            };
                            let now = chrono::Utc::now().timestamp();
                            let start_ts = now - position as i64;
                            let end_ts = start_ts + duration as i64;
                            println!("DEBUG:::Current Track Info:");
                            println!("DEBUG:::State: {}", state);
                            println!("DEBUG:::Title: {}", title);
                            println!("DEBUG:::Artist: {}", artist);
                            println!("DEBUG:::Album: {}", album);
                            println!("DEBUG:::Duration: {} seconds", duration);
                            println!("DEBUG:::Position: {} seconds", position);
                            println!("DEBUG:::(Last Position: {} seconds)", last_position);
                            println!("DEBUG:::Album Art URL: {}", public_url);
                            
                            match state {
                                "playing" => {
                                    println!("DEBUG:::State is 'playing'.");
                                    if 
                                        last_track != current_track || 
                                        last_position > position || 
                                        position > last_position + 5.0 ||
                                        last_state != "playing" {
                                        println!("DEBUG:::Setting activity: {} - {} [{}]", artist, title, album);
                                        discord.set_activity(
                                            state,
                                            title,
                                            artist,
                                            album,
                                            &public_url,
                                            start_ts,
                                            end_ts,
                                        );
                                    }
                                },
                                "paused" => {
                                    if last_state == "paused" {
                                        println!("\nTHREAD:::State is already 'paused', ending loop.");
                                        println!("-----------------------------------------------------------\n");
                                        continue;
                                    }
                                    println!("DEBUG:::State is 'paused'\nDEBUG:::Setting activity to 'paused' state.");
                                    discord.set_activity(
                                        state,
                                        title,
                                        artist,
                                        album,
                                        &public_url,
                                        start_ts,
                                        end_ts,
                                    );
                                },
                                "stopped" => {
                                    if last_state == "stopped" {
                                        println!("\nTHREAD:::State is already 'stopped', ending loop.");
                                        println!("-----------------------------------------------------------\n");
                                        continue;
                                    }
                                    println!("DEBUG:::State is 'stopped'\nDEBUG:::Clearing activity.");
                                    discord.clear();
                                }
                                _ => println!("ERROR:::Unknown track state: {}", state),
                            }
                            last_position = position;
                            last_track = current_track;
                            last_state = state.to_string();
                        }  
                    }
                }
                println!("\nTHREAD:::Loop iteration ended, sleeping for 3 seconds.");
                println!("-----------------------------------------------------------\n");
                thread::sleep(Duration::from_secs(3));
            }
        });
    }
    
    tauri::Builder::default()
    .setup(|app| {
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        let title_i = MenuItem::with_id(app, "title", "Apple Music Discord RPC", false, None::<&str>)?;
        let msg_i = MenuItem::with_id(app, "message", "made with love from Squishy <3", false, None::<&str>)?;
        let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&title_i, &msg_i, &quit_i])?;
    
        let _tray = TrayIconBuilder::new()
            .menu(&menu)
            .icon(app.default_window_icon().unwrap().clone())
            .on_menu_event(|app, event| match event.id.as_ref() {
                "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
                }
                _ => {
                println!("menu item {:?} not handled", event.id);
                }
            })
            .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_current_track])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }

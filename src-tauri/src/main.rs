// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hypixel;
pub mod log_regex;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, hypixel::get_latest_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

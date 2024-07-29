// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
mod hypixel;
pub mod libs;
pub mod log_regex;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hypixel::get_latest_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

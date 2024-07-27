// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hypixel;
pub mod log_regex;

use std::time::Instant;

fn main() {
    let log_path = "/Users/liangmi/.lunarclient/logs/game";
    let username = "liangmimi";

    for _ in 0..5 {
        let start_time = Instant::now();

        hypixel::get_latest_info(log_path, username);

        let elapsed = start_time.elapsed();
        println!("All Things Run Time: {:?}", elapsed);
    }

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![hypixel::get_latest_info])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

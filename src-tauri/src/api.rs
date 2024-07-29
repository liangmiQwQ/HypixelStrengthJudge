use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    vec,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::api::path::app_cache_dir;

#[derive(Deserialize, Serialize, Clone)]
struct CacheFile {
    data: Vec<CachePlayerData>,
}

#[derive(Deserialize, Serialize, Clone)]
struct CachePlayerData {
    data: Option<PlayerData>,
    time: i64,
}

use crate::{
    hypixel::{PlayerData, Rank},
    libs::{current_timestamp, get_rank_color, rgb_to_hex},
};

pub async fn get_player_data(
    app_handle: tauri::AppHandle,
    api_key: String,
    username: String,
    delay: i64,
) -> Option<PlayerData> {
    let mut cache_file = get_cache_file(app_handle.clone());
    if let Some(cache_data) = cache_file.iter().find(|&cache_player_data| {
        if let Some(player_data) = cache_player_data.data.clone() {
            let now = current_timestamp();
            if now - cache_player_data.time < delay && player_data.name == username {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }) {
        return cache_data.data.clone();
    } else {
        // let's get a little more exciting and get the hypixel api
        let mut player_data = PlayerData {
            name: username.clone(), // √
            rank: Rank {
                name: "DEFAULT".to_string(),
                plus_color: None,
                name_color: rgb_to_hex(170, 170, 170),
            },
            bw_fkdr: "0.00".to_string(), // √
            bw_level: 0,                 // √️
            lobby_level: 0,              // √
            bblr: "0.00".to_string(),    // √
            win_streak: 0,               // √
        };

        let option_response = match reqwest::get(format!(
            "https://api.hypixel.net/player?key={}&name={}",
            api_key, username
        ))
        .await
        {
            Ok(data) => match data.json::<Value>().await {
                Ok(result) => Some(result),
                Err(e) => {
                    println!("Something have error {}", e);
                    return None;
                }
            },
            Err(e) => {
                println!("Something have error {}", e);
                return None;
            }
        };
        if let Some(response) = option_response {
            let player_data_server = response["player"].clone();
            if !player_data_server.is_null() {
                // lobby level
                if let Some(network_exp) = player_data_server
                    .get("networkExp")
                    .and_then(|network_exp| network_exp.as_u64())
                {
                    player_data.lobby_level = get_hypixel_lobby_level(network_exp)
                }

                // bw level
                if let Some(achievements) = player_data_server.get("achievements") {
                    if let Some(bedwars_level) = achievements
                        .get("bedwars_level")
                        .and_then(|lvl| lvl.as_u64())
                    {
                        player_data.bw_level = bedwars_level as u16
                    }
                }

                // states (bblr fkdr win_streak)
                if let Some(stats) = player_data_server.get("stats") {
                    if let Some(bw) = stats.get("Bedwars") {
                        // bedwars value
                        let mut fk: u64 = 0;
                        let mut fd: u64 = 0;
                        if let Some(final_kill) =
                            bw.get("final_kills_bedwars").and_then(|fk| fk.as_u64())
                        {
                            fk = final_kill.clone();
                        }
                        if let Some(final_death) =
                            bw.get("final_deaths_bedwars").and_then(|fd| fd.as_u64())
                        {
                            fd = final_death.clone();
                        }
                        if fd != 0 {
                            player_data.bw_fkdr = format!("{:.2}", fk as f64 / fd as f64);
                        }

                        let mut bb: u64 = 0; // beds_broken_bedwars
                        let mut bl: u64 = 0; // beds_lost_bedwars
                        if let Some(beds_broken_bedwars) =
                            bw.get("beds_broken_bedwars").and_then(|bb| bb.as_u64())
                        {
                            bb = beds_broken_bedwars
                        }
                        if let Some(beds_lost_bedwars) =
                            bw.get("beds_lost_bedwars").and_then(|bl| bl.as_u64())
                        {
                            bl = beds_lost_bedwars
                        }
                        if bl != 0 {
                            player_data.bblr = format!("{:.2}", bb as f64 / bl as f64)
                        }

                        if let Some(win_streak) = bw.get("winstreak").and_then(|ws| ws.as_u64()) {
                            player_data.win_streak = win_streak;
                        }
                    }
                }

                // rank
                let rank_colors = get_rank_color();
                if username == "Technoblade" {
                    println!("Technoblade never dies");
                    player_data.rank.name = "PIG++".to_string();
                    player_data.rank.plus_color = Some(rgb_to_hex(85, 255, 255));
                    player_data.rank.name_color = rgb_to_hex(255, 85, 255);
                } else if username == "hypixel" || username == "Rezzus" {
                    player_data.rank.name = "OWNER".to_string();
                    player_data.rank.name_color = rgb_to_hex(255, 85, 85)
                } else {
                    let mut hypixel_rank: String = "NORMAL".to_string();
                    if let Some(rank) = player_data_server
                        .get("rank")
                        .and_then(|rank| rank.as_str())
                    {
                        hypixel_rank = rank.to_string();
                    } else {
                        if let Some(monthly_package_rank) = player_data_server
                            .get("monthlyPackageRank")
                            .and_then(|rank| rank.as_str())
                        {
                            if monthly_package_rank == "SUPERSTAR" {
                                hypixel_rank = String::from("SUPERSTAR");
                            } else {
                                if let Some(new_package_rank) = player_data_server
                                    .get("newPackageRank")
                                    .and_then(|rank| rank.as_str())
                                {
                                    hypixel_rank = new_package_rank.to_string()
                                }
                            }
                        } else if let Some(new_package_rank) = player_data_server
                            .get("newPackageRank")
                            .and_then(|rank| rank.as_str())
                        {
                            hypixel_rank = new_package_rank.to_string();
                        }
                    }
                    // plus color
                    if hypixel_rank == "MVP_PLUS" || hypixel_rank == "SUPERSTAR" {
                        if let Some(rank_plus_color) = player_data_server
                            .get("rankPlusColor")
                            .and_then(|rpc| rpc.as_str())
                        {
                            if let Some(&plus_color_hex) = rank_colors
                                .to_map()
                                .get(rank_plus_color.to_lowercase().as_str())
                            {
                                player_data.rank.plus_color = Some(plus_color_hex.clone())
                            }
                        } else {
                            player_data.rank.plus_color = Some(rank_colors.red.clone())
                        }
                    }

                    if hypixel_rank == "VIP" {
                        player_data.rank.name = "VIP".to_string();
                        player_data.rank.name_color = rank_colors.green.clone()
                    } else if hypixel_rank == "VIP_PLUS" {
                        player_data.rank.plus_color = Some(rank_colors.gold);
                        player_data.rank.name = "VIP+".to_string();
                        player_data.rank.name_color = rank_colors.green.clone();
                    } else if hypixel_rank == "MVP" {
                        player_data.rank.name = "MVP".to_string();
                        player_data.rank.name_color = rgb_to_hex(85, 255, 255)
                    } else if hypixel_rank == "MVP_PLUS" {
                        player_data.rank.name = "MVP+".to_string();
                        player_data.rank.name_color = rgb_to_hex(85, 255, 255);
                        // plus color not needed
                    } else if hypixel_rank == "SUPERSTAR" {
                        player_data.rank.name = "MVP++".to_string();
                        player_data.rank.name_color = rank_colors.gold.clone()
                    }
                }
                cache_file.push(CachePlayerData {
                    time: current_timestamp(),
                    data: Some(player_data.clone()),
                });
            } else {
                cache_file.push(CachePlayerData {
                    time: current_timestamp(),
                    data: None,
                });
                return None; // Nick
            }

            save_cache_file(app_handle.clone(), cache_file);
            // player_data
            Some(player_data)
        } else {
            return None; // server error
        }
    };
    None
}

fn get_hypixel_lobby_level(network_exp: u64) -> u16 {
    ((0.0008 * network_exp as f64 + 12.25).sqrt() + -3.5 + 1.0).round() as u16
}

fn create_cache_file(cache_dir: PathBuf) -> Option<File> {
    let file_path = cache_dir.join("api_cache.json");

    match File::create(file_path) {
        Ok(file) => Some(file),
        Err(e) => {
            println!("Can't create cache file {}", e);
            None
        }
    }
}

fn get_cache_file(app_handle: tauri::AppHandle) -> Vec<CachePlayerData> {
    if let Some(cache_dir) = app_cache_dir(&app_handle.config()) {
        let mut file_contains = String::new();
        match File::open(cache_dir.join("api_cache.json")) {
            Ok(mut file) => match file.read_to_string(&mut file_contains) {
                Ok(_) => (),
                Err(_) => (),
            },
            Err(_) => match create_cache_file(cache_dir) {
                Some(mut file) => match file.read_to_string(&mut file_contains) {
                    Ok(_) => (),
                    Err(_) => (),
                },
                None => (),
            },
        };

        let cache_file_vec: CacheFile = match serde_json::from_str(file_contains.as_str()) {
            Ok(cache_file_vec) => cache_file_vec,
            Err(_e) => CacheFile { data: vec![] },
        };

        return cache_file_vec.data.clone();
    } else {
        return vec![];
    };
}

fn save_cache_file(app_handle: tauri::AppHandle, value: Vec<CachePlayerData>) {
    if let Some(cache_dir) = app_cache_dir(&app_handle.config()) {
        let cache_path = cache_dir.join("api_cache.json");
        let cache_file = CacheFile { data: value };
        let json_data = serde_json::to_string(&cache_file).unwrap();

        let _ = fs::write(cache_path, json_data);
    }
}

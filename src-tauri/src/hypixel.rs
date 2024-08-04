use crate::api::get_player_data;
use crate::libs::current_timestamp;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

use crate::log_regex::{
    extract_party_leader, extract_party_members, extract_party_moderators, get_job_change_patterns,
    get_party_join_patterns, get_party_leave_patterns, get_player_join_patterns,
    get_player_leave_patterns, get_useful_party_lines_patterns, get_useful_player_lines_patterns,
    get_user_leave_patterns,
};

#[derive(Serialize, Debug)]
pub struct PartyInfo {
    user_job: String, // leader other
    players: Vec<PartyPlayerData>,
}

#[derive(Serialize, Debug, Clone)]
struct PartyPlayerData {
    name: String,
    player_data: Option<PlayerData>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Location {
    game_type: String,         // "BEDWARS"
    server_type: String, // "LOBBY" or "GAME", if "server" starts with "dynamiclobby", it's "LOBBY"
    game_mode: Option<String>, // "BEDWARS_FOUR_FOUR" etc.
    map: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawLocation {
    server: String,
    gametype: String,
    mode: Option<String>,
    map: Option<String>,
    lobbyname: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerData {
    pub name: String,
    pub rank: Rank,
    pub bw_fkdr: String, // rust decimal problem so use string(format!)
    pub bw_level: u16,
    pub lobby_level: u16,
    pub bblr: String, // the same reason as bw_fkdr
    pub win_streak: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReturnPlayerData {
    pub name: String,
    pub data: Option<PlayerData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rank {
    pub name: String,               // vip vip+ default
    pub name_color: String,         // #xxxxxx
    pub plus_color: Option<String>, // #xxxxxx
}

struct LogFilePath {
    path: String,
    timestamp: i64,
}

struct LogFile {
    last_line_number: usize,
    useful_line: UsefulLines,
}

struct LatestFile {
    path: String,
    gap: i64,
}

#[derive(Clone, Debug)]
struct UsefulLines {
    pl_lines: Vec<String>,
    party_lines: Vec<String>,
    location_line: Option<String>,
    player_lines: Vec<String>, // someone joined the game, some one have left the game
    who_line: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ReturnData {
    pub player_data: Vec<ReturnPlayerData>,
    // Option nesting
    // outside option: maybe the player isn't in any games
    // inside option: maybe some players using Nick
    pub personal_data: PersonalData,
    pub party_info: Option<PartyInfo>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PersonalData {
    pub location: Location,
    pub data: Option<PlayerData>,
}

struct PlayerDataHandle {
    handle: Pin<Box<dyn Future<Output = ()> + Send>>,
    player_name: String,
    data_type: String,
}

lazy_static! {
    static ref LOCATION: Mutex<String> = Mutex::new(String::from("LOBBY"));
    static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref PLAYERS_DATA: Mutex<Vec<PlayerData>> = Mutex::new(vec![]);
    static ref USER_ID: Mutex<String> = Mutex::new(String::from(""));
    static ref PARTY_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref LATEST_LOG_FILE_PATH: StdMutex<LogFilePath> = StdMutex::new(LogFilePath {
        path: String::from("unknown"),
        timestamp: current_timestamp(),
    });
    static ref LATEST_LOG_FILE: StdMutex<LogFile> = StdMutex::new(LogFile {
        last_line_number: 0,
        useful_line: UsefulLines {
            pl_lines: vec![],
            location_line: None,
            party_lines: vec![],
            player_lines: vec![],
            who_line: None
        }
    });
}

#[tauri::command]
pub async fn get_latest_info(
    log_dir_path: &str,
    username: &str,
    app_handle: tauri::AppHandle,
    api_key: String,
) -> Result<ReturnData, ()> {
    let start_time = Instant::now();
    let mut return_data = ReturnData {
        player_data: Vec::new(),
        personal_data: PersonalData {
            location: Location {
                game_type: String::from("UNKNOWN"),
                server_type: String::from("UNKNOWN"),
                game_mode: None,
                map: None,
            },
            data: None,
        },

        party_info: None,
    };

    // let location_re = Regex::new(r#"\{"server":"[^"]+","gametype":"[^"]+".*}"#).unwrap();

    let useful_lines = get_useful_lines(log_dir_path);

    let mut handles: Vec<PlayerDataHandle> = vec![];
    let arc_party_info_players: Arc<Mutex<Vec<PartyPlayerData>>> = Arc::new(Mutex::new(Vec::new()));
    let arc_players: Arc<Mutex<Vec<ReturnPlayerData>>> = Arc::new(Mutex::new(Vec::new()));
    let arc_personal_data: Arc<Mutex<Option<PlayerData>>> = Arc::new(Mutex::new(None));
    // tokio::spawn✌️

    let is_pl: bool = !useful_lines.pl_lines.is_empty();
    let who_line: String = match useful_lines.who_line {
        Some(wl) => wl,
        None => "".to_string(),
    };
    let location_line: String = match useful_lines.location_line {
        Some(ll) => ll,
        None => "".to_string(),
    };

    if is_pl {
        let mut is_in_party = true;
        if useful_lines.pl_lines.iter().any(|message| {
            get_user_leave_patterns()
                .iter()
                .any(|pattern| pattern.is_match(message))
        }) {
            return_data.party_info = None;
            is_in_party = false;
        }

        if is_in_party {
            let leader_line = useful_lines.pl_lines[2].clone();
            if leader_line
                .to_uppercase()
                .contains(username.to_uppercase().as_str())
            {
                return_data.party_info = Some(PartyInfo {
                    user_job: "LEADER".to_string(),
                    players: Vec::new(),
                });
                let username_clone = username.to_string();
                let api_key_clone = api_key.clone();
                let app_handle_clone = app_handle.clone();
                let arc_players = Arc::clone(&arc_party_info_players);

                handles.push(PlayerDataHandle {
                    player_name: username.to_string(),
                    data_type: "PARTY".to_string(),
                    handle: Box::pin(async move {
                        // println!("++++++++++++++发现真寻（组队）++++++++++++++");
                        // let mut players = arc_players.lock().unwrap();
                        let username = username_clone.clone();
                        let player_data = get_player_data(
                            app_handle_clone,
                            api_key_clone,
                            username.clone(),
                            5 * 60 * 60 * 1000,
                        )
                        .await;

                        let mut players = arc_players.lock().await;
                        players.push(PartyPlayerData {
                            name: username,
                            player_data,
                        });
                    }),
                });
            } else {
                let leader = extract_party_leader(leader_line.as_str())
                    .unwrap_or_else(|| "some error".to_string());
                if leader != "some error" {
                    return_data.party_info = Some(PartyInfo {
                        user_job: "OTHER".to_string(),
                        players: Vec::new(),
                    });
                    let api_key_clone = api_key.clone();
                    let app_handle_clone = app_handle.clone();
                    let leader_clone = leader.clone();
                    let arc_players = Arc::clone(&arc_party_info_players);

                    handles.push(PlayerDataHandle {
                        player_name: leader.clone(),
                        data_type: "PARTY".to_string(),
                        handle: Box::pin(async move {
                            let leader_name = leader_clone.clone();

                            let player_data = get_player_data(
                                app_handle_clone,
                                api_key_clone,
                                leader_name.clone(),
                                3 * 60 * 60 * 1000,
                            )
                            .await;

                            let mut players = arc_players.lock().await;
                            players.push(PartyPlayerData {
                                name: leader_name,
                                player_data,
                            });
                        }),
                    });
                }
            }; // the leader line

            // six times run
            for add_number in 0..6 {
                let next_line = useful_lines.pl_lines[1 + add_number].clone();
                if next_line.contains("Party Moderators:") {
                    let moderators: Vec<_> = match extract_party_moderators(next_line.as_str()) {
                        Some(moderators) => moderators,
                        None => vec![],
                    };

                    for moderator in moderators.iter() {
                        let app_handle_clone = app_handle.clone();
                        let api_key_clone = api_key.clone();
                        let moderator_clone = moderator.clone();
                        let arc_players = Arc::clone(&arc_party_info_players);

                        handles.push(PlayerDataHandle {
                            player_name: moderator.clone(),
                            data_type: "PARTY".to_string(),
                            handle: Box::pin(async move {
                                let moderator_name = moderator_clone.clone();
                                let player_data = get_player_data(
                                    app_handle_clone,
                                    api_key_clone,
                                    moderator_name.clone(),
                                    3 * 60 * 60 * 1000,
                                )
                                .await;

                                let mut players = arc_players.lock().await;
                                players.push(PartyPlayerData {
                                    name: moderator_name,
                                    player_data,
                                });
                            }),
                        });
                    }
                } else if next_line.contains("Party Members:") {
                    let members = match extract_party_members(next_line.as_str()) {
                        Some(members) => members,
                        None => vec![],
                    };

                    for member in members.iter() {
                        let app_handle_clone = app_handle.clone();
                        let api_key_clone = api_key.clone();
                        let member_clone = member.clone();
                        let arc_players = Arc::clone(&arc_party_info_players);

                        handles.push(PlayerDataHandle {
                            player_name: member.clone(),
                            data_type: "PARTY".to_string(),
                            handle: Box::pin(async move {
                                let member_name = member_clone.clone();

                                let player_data = get_player_data(
                                    app_handle_clone,
                                    api_key_clone,
                                    member_name.clone(),
                                    3 * 60 * 60 * 1000,
                                )
                                .await;

                                let mut players = arc_players.lock().await;
                                players.push(PartyPlayerData {
                                    name: member_name,
                                    player_data,
                                });
                            }),
                        })
                    }
                }
            }

            // Processing useful information
            let party_join_patterns = get_party_join_patterns();
            let party_leave_patterns = get_party_leave_patterns();
            let job_change_patterns = get_job_change_patterns();

            for message in useful_lines.party_lines.iter() {
                let mut is_message_used = false;

                // someone joined
                for pattern in party_join_patterns.clone() {
                    // to do (use is_match)
                    if pattern.is_match(message) {
                        if let Some(join_player) = pattern
                            .captures(message)
                            .and_then(|caps| caps.get(1))
                            .map(|match_| match_.as_str().to_string())
                        {
                            is_message_used = true;

                            let app_handle_clone = app_handle.clone();
                            let api_key_clone = api_key.clone();
                            let join_player_clone = join_player.clone();
                            let arc_players = Arc::clone(&arc_party_info_players);

                            handles.push(PlayerDataHandle {
                                player_name: join_player.clone(),
                                data_type: "PARTY".to_string(),
                                handle: Box::pin(async move {
                                    let join_player_name = join_player_clone.clone();

                                    let player_data = get_player_data(
                                        app_handle_clone,
                                        api_key_clone,
                                        join_player_name.clone(),
                                        3 * 60 * 60 * 1000,
                                    )
                                    .await;

                                    let mut players = arc_players.lock().await;
                                    players.push(PartyPlayerData {
                                        name: join_player_name,
                                        player_data,
                                    });
                                }),
                            })
                        }
                    }
                }

                if !is_message_used {
                    // some one left
                    for pattern in party_leave_patterns.clone() {
                        if pattern.is_match(message) {
                            if let Some(leave_player) = pattern
                                .captures(message)
                                .and_then(|caps| caps.get(1))
                                .map(|match_| match_.as_str().to_string())
                            {
                                is_message_used = true;
                                // handles =
                                // handles.iter().handles.retain(|handle: &PlayerDataHandle| {
                                //     handle.player_name != leave_player
                                // });
                                handles.retain(|handle| {
                                    if handle.data_type == "PARTY" {
                                        handle.player_name != leave_player
                                    } else {
                                        true
                                    }
                                });
                                // leave_player_name.push(leave_player)
                            }
                        }
                    }
                }

                if !is_message_used {
                    for pattern in job_change_patterns.clone() {
                        if pattern.is_match(message) {
                            if let Some(leader_player) = pattern
                                .captures(message)
                                .and_then(|caps| caps.get(1))
                                .map(|match_| match_.as_str().to_string())
                            {
                                if let Some(party_info) = &mut return_data.party_info {
                                    if leader_player.to_uppercase() == username.to_uppercase() {
                                        // the user is the leader
                                        party_info.user_job = String::from("LEADER")
                                    } else {
                                        party_info.user_job = String::from("OTHER")
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // delete urself (not only in the party but also in the game!!!)
            handles.retain(|handle| {
                if handle.data_type == "PARTY" {
                    handle.player_name != username
                } else if handle.data_type == "GAME" {
                    handle.player_name != username
                } else {
                    true
                }
            });
            // ⬆️ No ZhenXun_awa
        }
    };
    if who_line != "" {
        if let Some(pos) = who_line.find("[CHAT] ONLINE:") {
            let players_str_no_space = who_line[pos + "[CHAT] ONLINE:".len()..].replace(" ", "");
            let players: Vec<&str> = players_str_no_space.split(",").collect();

            // let formatted_players = players.join(", ");
            // println!("Players: {}", formatted_players);
            // ⬆️ success

            let party_players: Vec<String> = handles
                .iter()
                .map(|handle| handle.player_name.clone())
                .collect();
            for player in players {
                // each player
                // exclude party players
                let mut is_party_player = false;
                for party_player_name in &party_players {
                    if party_player_name == player {
                        is_party_player = true;
                        break;
                    }
                }
                // println!("{} = {}", !is_party_player, player);

                if !is_party_player {
                    // thread start!
                    let player_name_clone = player.to_string();
                    let api_key_clone = api_key.clone();
                    let app_handle_clone = app_handle.clone();
                    let players_arc = Arc::clone(&arc_players);

                    handles.push(PlayerDataHandle {
                        player_name: player.to_string(),
                        data_type: "GAME".to_string(),
                        handle: Box::pin(async move {
                            let username = player_name_clone.clone();

                            let player_data = get_player_data(
                                app_handle_clone,
                                api_key_clone,
                                username.clone(),
                                24 * 60 * 60 * 1000,
                            )
                            .await;

                            let mut players = players_arc.lock().await;
                            players.push(ReturnPlayerData {
                                name: username,
                                data: player_data,
                            })
                        }),
                    })
                }
            }

            let player_join_patterns = get_player_join_patterns();
            let player_leave_patterns = get_player_leave_patterns();
            // (someone joined someone left todo)
            for message in useful_lines.player_lines.iter() {
                let mut is_message_used = false;

                // join
                for pattern in player_join_patterns.clone() {
                    if pattern.is_match(message) {
                        if let Some(join_player) = pattern
                            .captures(message)
                            .and_then(|caps| caps.get(1))
                            .map(|match_| match_.as_str().to_string())
                        {
                            is_message_used = true;

                            let player_name_clone = join_player.to_string();
                            let api_key_clone = api_key.clone();
                            let app_handle_clone = app_handle.clone();
                            let players_arc = Arc::clone(&arc_players);

                            handles.push(PlayerDataHandle {
                                player_name: join_player.clone(),
                                handle: Box::pin(async move {
                                    let join_player_name = player_name_clone.clone();

                                    let player_data = get_player_data(
                                        app_handle_clone,
                                        api_key_clone,
                                        join_player_name.clone(),
                                        24 * 30 * 60 * 1000,
                                    )
                                    .await;

                                    let mut players = players_arc.lock().await;

                                    players.push(ReturnPlayerData {
                                        name: join_player_name,
                                        data: player_data,
                                    })
                                }),
                                data_type: "GAME".to_string(),
                            })
                        }
                    }
                }

                if !is_message_used {
                    // some player leave
                    for pattern in player_leave_patterns.clone() {
                        if pattern.is_match(message) {
                            if let Some(left_player) = pattern
                                .captures(message)
                                .and_then(|caps| caps.get(1))
                                .map(|match_| match_.as_str().to_string())
                            {
                                handles.retain(|handle| {
                                    if handle.data_type == "GAME" {
                                        handle.player_name != left_player
                                    } else {
                                        true
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    if location_line != "" {
        if let Some(pos) = Regex::new(r#"\{"server""#)
            .unwrap()
            .find(location_line.as_str())
        {
            let start = pos.start();
            let location_str = &location_line[start..];

            if let Some(raw_location) = match serde_json::from_str::<RawLocation>(location_str) {
                Ok(rl) => Some(rl),
                Err(_) => None,
            } {
                // gametype
                return_data.personal_data.location.game_type = raw_location.gametype;
                // server_type
                if raw_location.server.starts_with("dynamiclobby") {
                    // in the lobby
                    return_data.personal_data.location.server_type = "LOBBY".to_string();
                } else {
                    return_data.personal_data.location.server_type = "GAME".to_string();
                }
                // game mode
                if let Some(game_mode) = raw_location.mode {
                    return_data.personal_data.location.game_mode = Some(game_mode);
                }
                if let Some(map) = raw_location.map {
                    return_data.personal_data.location.map = Some(map);
                }
            } // else don't do anything, frontend return needJoinServer
        }
    }

    // get personal data
    let app_handle_clone = app_handle.clone();
    let api_key_clone = api_key.clone();
    let username_clone = username.to_string();
    let arc_user_data = Arc::clone(&arc_personal_data);
    handles.push(PlayerDataHandle {
        player_name: username.to_string() + " clone",
        data_type: "PERSONAL".to_string(),
        // not applicable for personal data
        handle: Box::pin(async move {
            let username = username_clone.clone();

            let player_data = get_player_data(
                app_handle_clone,
                api_key_clone,
                username,
                2 * 60 * 60 * 1000,
            )
            .await;

            let mut personal_data = arc_user_data.lock().await;

            *personal_data = player_data;
        }),
    });

    // do all thread
    let mut futures = Vec::new();
    for player_data_handle in handles {
        futures.push(tokio::spawn(player_data_handle.handle));
        // player_data_handle.handle.join().unwrap();
    }
    for future in futures {
        future.await.unwrap();
        // wait all
    }

    if let Some(party_info) = &mut return_data.party_info {
        let players: tokio::sync::MutexGuard<Vec<PartyPlayerData>> =
            arc_party_info_players.lock().await;

        for player in players.iter() {
            party_info.players.push(player.clone())
        }
    }

    let personal_data = &mut return_data.personal_data;
    let option_personal_data: tokio::sync::MutexGuard<Option<PlayerData>> =
        arc_personal_data.lock().await;
    personal_data.data = option_personal_data.clone();

    let players: tokio::sync::MutexGuard<Vec<ReturnPlayerData>> = arc_players.lock().await;

    println!("Player List Start!");
    for player in players.iter() {
        println!("{:?}", player);
        return_data.player_data.push(player.clone());
    }
    println!("Player List End!");

    let elapsed: std::time::Duration = start_time.elapsed();
    println!(
        "[Strength Judge] [info] Getting latest info in {:?}",
        elapsed
    );
    Ok(return_data)
}

fn get_latest_log_file(log_dir_path: &str) -> String {
    let message: String = match fs::read_to_string(get_latest_log_path(log_dir_path)) {
        Ok(file) => file,
        Err(e) => format!("Error {}", e),
    };
    message
}

fn get_useful_lines(log_dir_path: &str) -> UsefulLines {
    let mut latest_log_file = LATEST_LOG_FILE.lock().unwrap();
    let file_content = get_latest_log_file(log_dir_path);
    let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    let useful_party_lines_patterns = get_useful_party_lines_patterns();
    let mut is_pl = false; // if it = 2 -> break
    let mut is_who = false;
    let mut is_location = true;

    let location_pattern = Regex::new(r#"\{"server":"[^"]*","gametype":"[^"]*""#).unwrap();
    // let location_pattern = Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)离开了组队。").unwrap();
    let party_patterns = useful_party_lines_patterns.clone();
    let player_patterns = get_useful_player_lines_patterns().clone();
    let reversed_lines: Vec<&String> = lines.iter().rev().collect();
    let mut addon_useful_party_lines: Vec<String> = Vec::new();
    let mut addon_useful_player_lines: Vec<String> = Vec::new();
    for (index, line) in reversed_lines.iter().enumerate() {
        // two method to break
        // 1. pl, location(todo) and players(todo)'s lens all > 0 or != "" && from new lines
        // 2. lines.len() - latest_log_file.last_line_number - 1 (the rev line of the last line) <= index
        /*
        1 something 0             1 something 0
        2 something 1             2 nothing   1                         real_index = all_line_len - rev_index - 1
        3 something 2 ==reverse=> 3 something 2 => {index:1, len: 5} => or
        4 nothing   3             4 something 3                         rev_index = all_line_len - real_index - 1
        5 something 4             5 something 4
        */
        if lines.len() - latest_log_file.last_line_number - 1 <= index || (is_pl && is_location)
        // the who line before location line is meaningless
        // || (is_pl && is_who)
        // although pl and who are all filled, but location line is also needed
        {
            break;
        } else {
            // PL line
            if line.contains("[CHAT] Party Members ") && !is_pl {
                //pl lines
                let real_index = lines.len() - index - 1;
                // Ensure we do not exceed the bounds of the lines vector
                latest_log_file.useful_line.pl_lines.clear();
                for add_number in 0..7 {
                    if real_index + add_number < lines.len() {
                        latest_log_file
                            .useful_line
                            .pl_lines
                            .push(lines[real_index + add_number].clone());
                    } else {
                        // Exit the loop if we exceed the bounds
                        break;
                    }
                }
                is_pl = true
            } else if !is_who && line.contains("[CHAT] ONLINE: ") {
                latest_log_file.useful_line.who_line = Some(line.to_string());
                is_who = true
                // todo: who line
                // need only one who line
            } else if location_pattern.is_match(line) {
                latest_log_file.useful_line.location_line = Some(line.to_string());
                is_location = true
                // todo: location line
                // if pl line and location line are all found => break
            } else {
                if !is_pl {
                    for pattern in &party_patterns {
                        if pattern.is_match(&line) {
                            // latest_log_file
                            //     .useful_line
                            //     .party_lines
                            //     .push(line.to_string());
                            // ⬆️ that is error
                            addon_useful_party_lines.push(line.to_string());
                            break;
                        }
                    }
                } else if !is_who {
                    // all message after is_who
                    for pattern in &player_patterns {
                        if pattern.is_match(&line) {
                            addon_useful_player_lines.push(line.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }
    addon_useful_party_lines.reverse();
    addon_useful_player_lines.reverse();
    latest_log_file.last_line_number = lines.len() - 1;
    latest_log_file
        .useful_line
        .party_lines
        .extend(addon_useful_party_lines);
    latest_log_file
        .useful_line
        .player_lines
        .extend(addon_useful_player_lines);

    return latest_log_file.useful_line.clone();
}

fn get_latest_log_path(log_dir_path: &str) -> String {
    let mut latest_log_file_path = LATEST_LOG_FILE_PATH.lock().unwrap();
    if (current_timestamp() - latest_log_file_path.timestamp) > 30_000
        || latest_log_file_path.path == "unknown"
    {
        // Get latest log file
        let mut log_files: Vec<PathBuf> = Vec::new();

        let files = match fs::read_dir(log_dir_path) {
            Ok(files) => files,
            Err(_e) => {
                print!("Have some error when read dir");
                return format!("Error");
            }
        };

        for file in files {
            let file = match file {
                Ok(file) => file,
                Err(_e) => {
                    print!("Have some error when read file");
                    return format!("Error");
                }
            };
            let path = file.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".log") {
                            log_files.push(path.clone());
                            if file_name_str == "latest.log" {
                                return path.to_str().unwrap_or("null").to_string();
                            }
                        }
                    }
                }
            }
        }

        let mut latest_file = LatestFile {
            path: String::from(""),
            gap: 0,
        };
        let now: i64 = current_timestamp();

        for file in log_files {
            let latest_change = match get_modification_time(&file) {
                Some(change_time) => {
                    let timestamp_milliseconds: i64 = change_time
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as i64;

                    timestamp_milliseconds
                }
                None => {
                    eprintln!("Failed to get modification time");
                    return format!("error");
                }
            };

            let gap = now - latest_change;
            if gap < latest_file.gap {
                //laster
                latest_file = LatestFile {
                    path: file.to_str().unwrap_or("null").to_string(),
                    gap,
                }
            };
        }
        *latest_log_file_path = LogFilePath {
            timestamp: now,
            path: latest_file.path.clone(),
        };
        latest_log_file_path.path.clone()
    } else {
        return latest_log_file_path.path.clone();
    }
}

fn get_modification_time(file_path: &PathBuf) -> Option<SystemTime> {
    // match fs::metadata(file_path) {
    //     Ok(metadata) => match metadata.modified() {
    //         Ok(mod_time) => Some(mod_time),
    //         Err(e) => {
    //             eprintln!("Error getting modification time for file {}: {}", file_path.display(), e);
    //             None
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Error getting metadata for file {}: {}", file_path.display(), e);
    //         None
    //     }
    // }
    let metadata = fs::metadata(file_path).ok()?;
    metadata.modified().ok()
}

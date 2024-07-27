use lazy_static::lazy_static;
use serde::Serialize;
use std::fs::{self};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::log_regex::{
    extract_party_leader, extract_party_members, extract_party_moderators, get_job_change_patterns,
    get_party_join_patterns, get_party_leave_patterns, get_useful_party_lines_patterns,
    get_user_leave_patterns,
};

#[derive(Serialize)]
pub struct PartyInfo {
    user_job: String, // leader other
    players: Vec<String>,
}

#[derive(Serialize)]
pub struct Location {
    game_type: String,
    server_type: String, // "LOBBY" or "GAME", if "server" starts with "dynamiclobby", it's "LOBBY"
    game_mode: Option<String>, // "BEDWARS_FOUR_FOUR" etc.
}

#[derive(Serialize)]
pub struct PlayerData {
    name: String,
    rank: String,
    bw_fkdr: f64,
    bw_level: u16,
    lobby_level: u16,
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

#[derive(Clone)]
struct UsefulLines {
    pl_lines: Vec<String>,
    party_lines: Vec<String>,
}

#[derive(Serialize)]
pub struct ReturnData {
    player_data: Option<PlayerData>,
    location: Location,
    party_info: Option<PartyInfo>,
}

lazy_static! {
    static ref LOCATION: Mutex<String> = Mutex::new(String::from("LOBBY"));
    static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref PLAYERS_DATA: Mutex<Vec<PlayerData>> = Mutex::new(vec![]);
    static ref USER_ID: Mutex<String> = Mutex::new(String::from(""));
    static ref PARTY_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref LATEST_LOG_FILE_PATH: Mutex<LogFilePath> = Mutex::new(LogFilePath {
        path: String::from("unknown"),
        timestamp: current_timestamp(),
    });
    static ref LATEST_LOG_FILE: Mutex<LogFile> = Mutex::new(LogFile {
        last_line_number: 0,
        useful_line: UsefulLines {
            pl_lines: vec![],
            party_lines: vec![],
        }
    });
}

#[tauri::command]
pub fn get_latest_info(log_dir_path: &str, username: &str) -> ReturnData {
    let mut return_data = ReturnData {
        player_data: None,
        location: Location {
            game_type: String::from("UNKNOWN"),
            server_type: String::from("UNKNOWN"),
            game_mode: None,
        },
        party_info: None,
    };

    // let location_re = Regex::new(r#"\{"server":"[^"]+","gametype":"[^"]+".*}"#).unwrap();

    let useful_lines = get_useful_lines(log_dir_path);

    // --party list--
    // “[CHAT] Party Members” and the third line after that is Party leader
    // the fifth line is Party Members/Moderators
    // --join and left--
    // [someone joined the party.][someone加入了组队。]
    // [someone has left the party.][someone离开了组队.]
    // [You have joined someone's party!][你加入了someone]的组队！]
    // [You left the party.][你离开了组队。]
    // --kick--
    // [someone has been removed from the party.][someone已被移出组队。]
    // [You have been kicked from the party by someone][你已被someone踢出组队] // no "." or "。"
    // [Kicked someone because they were offline.][someone已断开连接， 被移出你的组队。]
    // --transfer--
    // [The party was transferred to someone by someone][someone将组队移交给了someone]//no "."or"。"
    // --disband--
    // [someone has disbanded the party!][someone解散了组队！]
    // [The party was disbanded because all invites expired and the party was empty.][因组队中没有成员， 且所有邀请均已过期， 组队已被解散。// --party list--
    // “[CHAT] Party Members” and the third line after that is Party leader
    // the fifth line is Party Members/Moderators
    // --join and left--
    // [someone joined the party.][someone加入了组队。]
    // [someone has left the party.][someone离开了组队.]
    // [You have joined someone's party!][你加入了someone]的组队！]
    // [You left the party.][你离开了组队。]
    // --kick--
    // [someone has been removed from the party.][someone已被移出组队。]
    // [You have been kicked from the party by someone][你已被someone踢出组队] // no "." or "。"
    // [Kicked someone because they were offline.][someone已断开连接， 被移出你的组队。]
    // --transfer--
    // [The party was transferred to someone by someone][someone将组队移交给了someone]//no "."or"。"
    // --disband--
    // [someone has disbanded the party!][someone解散了组队！]
    // [The party was disbanded because all invites expired and the party was empty.][因组队中没有成员， 且所有邀请均已过期， 组队已被解散。]

    let is_pl: bool = !useful_lines.pl_lines.is_empty(); // 修改1：简化判断

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
            // user used pl command
            // +2 and it's party leader
            let leader_line = useful_lines.pl_lines[2].clone();
            if leader_line.contains(username) {
                return_data.party_info = Some(PartyInfo {
                    players: vec![String::from(username)],
                    user_job: String::from("LEADER"),
                })
            } else {
                let leader = extract_party_leader(leader_line.as_str())
                    .unwrap_or_else(|| "some error".to_string());
                if leader != "some error" {
                    return_data.party_info = Some(PartyInfo {
                        players: vec![String::from(leader)],
                        user_job: String::from("OTHER"),
                    });
                }
            }; // the leader line

            // six times run
            for add_number in 0..6 {
                let next_line = useful_lines.pl_lines[1 + add_number].clone();
                if next_line.contains("Party Moderators:") {
                    let moderators = match extract_party_moderators(next_line.as_str()) {
                        Some(moderators) => moderators,
                        None => vec![],
                    };
                    if let Some(party_info) = &mut return_data.party_info {
                        party_info.players.extend(moderators);
                    }
                } else if next_line.contains("Party Members:") {
                    let members = match extract_party_members(next_line.as_str()) {
                        Some(members) => members,
                        None => vec![],
                    };
                    if let Some(party_info) = &mut return_data.party_info {
                        party_info.players.extend(members);
                    }
                }
            }
        }

        if is_in_party {
            // Processing useful information
            let party_join_patterns = get_party_join_patterns();
            let party_leave_patterns = get_party_leave_patterns();
            let job_change_patterns = get_job_change_patterns();

            let start_time = Instant::now();
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
                            if let Some(party_info) = &mut return_data.party_info {
                                party_info.players.push(join_player);
                            };
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
                                if let Some(party_info) = &mut return_data.party_info {
                                    party_info
                                        .players
                                        .retain(|player: &String| player != &leave_player);
                                };
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
                                    if leader_player == username {
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
            let elapsed = start_time.elapsed();
            println!("Find The Party Members Run Time: {:?}", elapsed);
        }
    };
    return_data
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
    // Get file value
    let patterns = useful_party_lines_patterns.clone();
    let reversed_lines: Vec<&String> = lines.iter().rev().collect();
    let mut addon_useful_party_lines: Vec<String> = Vec::new();
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
        if lines.len() - latest_log_file.last_line_number - 1 <= index {
            break;
        } else {
            if line.contains("[CHAT] Party Members ") {
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
                break; // some other things to add
            } else {
                for pattern in &patterns {
                    if pattern.is_match(&line) {
                        // latest_log_file
                        //     .useful_line
                        //     .party_lines
                        //     .push(line.to_string());
                        // ⬆️ that is error
                        addon_useful_party_lines.push(line.to_string());
                        break;
                    }
                    // for pattern in &patterns {
                    //     if pattern.is_match(&line) {
                    //         // latest_log_file
                    //         //     .useful_line
                    //         //     .party_lines
                    //         //     .push(line.to_string());
                    //         // ⬆️ that is error
                    //         addon_useful_party_lines.push(line.to_string());
                    //         break;
                    //     }
                    // }
                }
            }
        }
    }
    addon_useful_party_lines.reverse();
    latest_log_file.last_line_number = lines.len() - 1;
    latest_log_file
        .useful_line
        .party_lines
        .extend(addon_useful_party_lines);

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

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
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

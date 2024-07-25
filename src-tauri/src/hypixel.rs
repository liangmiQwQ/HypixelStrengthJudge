use lazy_static::lazy_static;
use std::fmt::format;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, File};


struct PlayerData {
    name: String,
    rank: String,
    bw_fkdr: f64,
    bw_level: u16,
    lobby_level: u16,
}
struct LogFile{
    path: String,
    timestamp: i64,
}

struct LatestFile{
    path: String,
    gap: i64,
}

lazy_static! {
    static ref LOCATION: Mutex<String> = Mutex::new(String::from("LOBBY"));
    static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref PLAYERS_DATA: Mutex<Vec<PlayerData>> = Mutex::new(vec![]);
    static ref USER_ID: Mutex<String> = Mutex::new(String::from(""));
    static ref PARTY_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref LATEST_LOG_FILE: Mutex<LogFile> = Mutex::new(LogFile {
        path: String::from("unknown"),
        timestamp: current_timestamp(),
    });
}

#[tauri::command]
fn get_latest_location(_logDirPath: &str) -> String {
    String::from("fuckyouRust")
}

fn get_latest_log_path(log_dir_path: &str) -> String {
    let mut latest_log_file = LATEST_LOG_FILE.lock().unwrap();
    if (current_timestamp() - latest_log_file.timestamp) > 60_000 || latest_log_file.path == "null" {
        // Get latest log file
        let mut log_files: Vec<PathBuf> = Vec::new();

        let files = match fs::read_dir(log_dir_path) {
            Ok(files)  => files,
            Err(_e) => {
                print!("Have some error when read dir");
                return format!("Error")
            }
        }; 

        for file in files{
            let file = match file {
                Ok(file) => file,
                Err(_e)=>{
                    print!("Have some error when read file");
                    return format!("Error")
                }  
            };
            let path = file.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name(){
                    if let Some(file_name_str) = file_name.to_str(){
                        if file_name_str.ends_with(".log"){
                            log_files.push(path.clone());
                            if file_name_str == "latest.log" {            
                                return path.to_str().unwrap_or("null").to_string()
                            }
                        }
                    }
                }
            }
        }

        let mut latest_file = LatestFile{
            path: String::from(""),
            gap: 0,
        };
        let now: i64 = current_timestamp();

        for file in log_files{
            let latest_change = match get_modification_time(&file) {
                Some(change_time) => {
                    let timestamp_milliseconds: i64 = change_time.duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as i64;
                    
                    timestamp_milliseconds
                },
                None => {
                    eprintln!("Failed to get modification time");
                    return format!("error")
                }
            };

            if latest_change - now < latest_file.gap { //laster
                latest_file = LatestFile{
                    path: file.to_str().unwrap_or("null").to_string(),
                    gap: latest_change - now
                }
            }
        }
        latest_file.path
    }else{
        return latest_log_file.path.clone(); 
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
use lazy_static::lazy_static;
use std::sync::Mutex;

struct PlayerData {
    name: String,
    rank: String,
    bw_fkdr: f64,
    bw_level: u16,
    lobby_level: u16,
}

lazy_static! {
    static ref LOCATION: Mutex<String> = Mutex::new(String::from("LOBBY"));
    static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref PLAYERS_DATA: Mutex<Vec<PlayerData>> = Mutex::new(vec![]);
}

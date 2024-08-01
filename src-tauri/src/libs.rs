use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct RankColors {
    pub gray: String,
    pub red: String,
    pub gold: String,
    pub green: String,
    pub yellow: String,
    pub aqua: String,
    pub light_purple: String,
    pub white: String,
    pub blue: String,
    pub dark_green: String,
    pub dark_red: String,
    pub dark_aqua: String,
    pub dark_purple: String,
    pub dark_gray: String,
    pub black: String,
    pub dark_blue: String,
}

impl RankColors {
    pub fn to_map(&self) -> HashMap<&str, &String> {
        let mut map = HashMap::new();
        map.insert("gray", &self.gray);
        map.insert("red", &self.red);
        map.insert("gold", &self.gold);
        map.insert("green", &self.green);
        map.insert("yellow", &self.yellow);
        map.insert("light_purple", &self.light_purple);
        map.insert("white", &self.white);
        map.insert("blue", &self.blue);
        map.insert("dark_green", &self.dark_green);
        map.insert("dark_red", &self.dark_red);
        map.insert("dark_aqua", &self.dark_aqua);
        map.insert("dark_purple", &self.dark_purple);
        map.insert("dark_gray", &self.dark_gray);
        map.insert("black", &self.black);
        map.insert("dark_blue", &self.dark_blue);
        map
    }
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn get_rank_color() -> RankColors {
    return RankColors {
        gray: "#aaaaaa".to_string(),
        aqua: "#14EAEA".to_string(),
        red: "#ff0000".to_string(),
        gold: "#ffaa00".to_string(),
        green: "#00E000".to_string(),
        yellow: "#ffff00".to_string(),
        light_purple: "#ff55ff".to_string(),
        white: "#ffffff".to_string(),
        blue: "#5555ff".to_string(),
        dark_green: "#00aa00".to_string(),
        dark_red: "#aa0000".to_string(),
        dark_aqua: "#00aaaa".to_string(),
        dark_purple: "#aa00aa".to_string(),
        dark_gray: "#555555".to_string(),
        black: "#000000".to_string(),
        dark_blue: "#0000aa".to_string(),
    };
}

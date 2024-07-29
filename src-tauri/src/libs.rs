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
        gray: rgb_to_hex(170, 170, 170),
        red: rgb_to_hex(255, 85, 85),
        gold: rgb_to_hex(255, 170, 0),
        green: rgb_to_hex(85, 255, 85),
        yellow: rgb_to_hex(255, 255, 85),
        light_purple: rgb_to_hex(255, 85, 255),
        white: rgb_to_hex(255, 255, 255),
        blue: rgb_to_hex(85, 85, 255),
        dark_green: rgb_to_hex(0, 170, 0),
        dark_red: rgb_to_hex(170, 0, 0),
        dark_aqua: rgb_to_hex(0, 170, 170),
        dark_purple: rgb_to_hex(170, 0, 170),
        dark_gray: rgb_to_hex(85, 85, 85),
        black: rgb_to_hex(0, 0, 0),
        dark_blue: rgb_to_hex(0, 0, 170),
    };
}

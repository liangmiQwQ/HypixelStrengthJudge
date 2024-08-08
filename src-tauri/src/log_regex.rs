use std::vec;

use regex::Regex;

pub fn get_useful_party_lines_patterns() -> Vec<Regex> {
    let mut patterns = Vec::new();

    patterns.extend(get_user_leave_patterns());
    patterns.extend(get_party_join_patterns());
    patterns.extend(get_party_leave_patterns());
    patterns.extend(get_job_change_patterns());

    patterns
}

pub fn get_useful_player_lines_patterns() -> Vec<Regex> {
    let mut patterns = Vec::new();

    patterns.extend(get_player_join_patterns());
    patterns.extend(get_player_leave_patterns());

    patterns
}

pub fn get_user_leave_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] You have joined (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)'s party!").unwrap(), // English
        Regex::new(r"\[CHAT\] 你加入了(?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)的组队！").unwrap(),          // Chinese
        Regex::new(r"\[CHAT\] You left the party\.").unwrap(),           // English
        Regex::new(r"\[CHAT\] 你离开了组队。").unwrap(),                 // Chinese
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) has been removed from the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)已被移出组队。").unwrap(),            // Chinese
        Regex::new(r"\[CHAT\] You have been kicked from the party by (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)").unwrap(), // English
        Regex::new(r"\[CHAT\] 你已被(?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)踢出组队").unwrap(),            // Chinese (no "." or "。")
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) has disbanded the party!").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)解散了组队！").unwrap(),              // Chinese
        Regex::new(
            r"\[CHAT\] The party was disbanded because all invites expired and the party was empty\.",
        )
        .unwrap(), // English
        Regex::new(r"\[CHAT\] 因组队中没有成员， 且所有邀请均已过期， 组队已被解散。").unwrap(), // Chinese
    ]
}

pub fn get_party_join_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) joined the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)加入了组队。").unwrap(), // Chinese
    ]
}

pub fn get_party_leave_patterns() -> Vec<Regex> {
    vec![
        Regex::new(
            r"\[CHAT\] Kicked (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) because they were offline\.",
        )
        .unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)已断开连接， 被移出你的组队。")
            .unwrap(), // Chinese
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) has left the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)离开了组队。").unwrap(), // Chinese
    ]
}
pub fn get_job_change_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] The party was transferred to (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) by (?:\[.*\])?(?:\s)?(?:[a-zA-Z0-9_]+)").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(?:[a-zA-Z0-9_]+)将组队移交给了(?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)").unwrap(), // Chinese (no "." or "。")
        Regex::new(r"\[CHAT\] The party was transferred to (?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+) because (?:\[.*\])?(?:\s)?(?:[a-zA-Z0-9_]+) left").unwrap(),
        Regex::new(r"\[CHAT\] 由于(?:\[.*\])?(?:\s)?(?:[a-zA-Z0-9_]+)离开了组队，组队被移交给了(?:\[.*\])?(?:\s)?([a-zA-Z0-9_]+)").unwrap()
    ]
}

pub fn get_player_join_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)加入了游戏").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+) has joined").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+) reconnected.").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)重新连接").unwrap(),
    ]
}

pub fn get_player_leave_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+) has quit").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)离开了游戏").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+) disconnected.").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)断开连接").unwrap(),
        // Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+) .*? FINAL KILL!").unwrap(),
        Regex::new(r"\[CHAT\] ([a-zA-Z0-9_]+)[\u4e00-\u9fa5].*? 最终击杀！").unwrap(), // CHINESE
    ]
}

pub fn extract_party_leader(line: &str) -> Option<String> {
    let re = Regex::new(r"\[CHAT\] Party Leader: (?:\[.*\])?(?:\s)?(\w*)").unwrap();

    re.captures(line)
        .and_then(|caps| caps.get(1))
        .map(|match_| match_.as_str().to_string())
}

pub fn extract_party_moderators(line: &str) -> Option<Vec<String>> {
    let re = Regex::new(r"\[CHAT\] Party Moderators: (.*)").unwrap();

    let players = re
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|match_| match_.as_str().to_string())?;

    let re_brackets = Regex::new(r"\[.*?\]").unwrap();
    let re_spaces = Regex::new(r"\s+").unwrap();
    let no_brackets = re_brackets.replace_all(&players, "");
    let result: Vec<String> = re_spaces
        .replace_all(&no_brackets, "")
        .split("●")
        .filter(|s| !s.is_empty()) // 过滤掉空字符串
        .map(|s| s.trim().to_string()) // 将 &str 转换为 String
        .collect();

    return Some(result);
}

pub fn extract_party_members(line: &str) -> Option<Vec<String>> {
    let re = Regex::new(r"\[CHAT\] Party Members: (.*)").unwrap();

    let players = re
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|match_| match_.as_str().to_string())?;

    let re_brackets = Regex::new(r"\[.*?\]").unwrap();
    let re_spaces = Regex::new(r"\s+").unwrap();
    let no_brackets = re_brackets.replace_all(&players, "");
    let result: Vec<String> = re_spaces
        .replace_all(&no_brackets, "")
        .split("●")
        .filter(|s| !s.is_empty()) // 过滤掉空字符串
        .map(|s| s.trim().to_string()) // 将 &str 转换为 String
        .collect();

    return Some(result);
}

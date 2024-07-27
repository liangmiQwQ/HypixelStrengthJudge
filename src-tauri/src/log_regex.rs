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

pub fn get_user_leave_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] You have joined (?:\[.*\])?(?:\s)?(.*)'s party!").unwrap(), // English
        Regex::new(r"\[CHAT\] 你加入了(?:\[.*\])?(?:\s)?(.*)的组队！").unwrap(),          // Chinese
        Regex::new(r"\[CHAT\] You left the party\.").unwrap(),           // English
        Regex::new(r"\[CHAT\] 你离开了组队。").unwrap(),                 // Chinese
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*) has been removed from the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)已被移出组队。").unwrap(),            // Chinese
        Regex::new(r"\[CHAT\] You have been kicked from the party by (?:\[.*\])?(?:\s)?(.*)").unwrap(), // English
        Regex::new(r"\[CHAT\] 你已被(?:\[.*\])?(?:\s)?(.*)踢出组队").unwrap(),            // Chinese (no "." or "。")
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*) has disbanded the party!").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)解散了组队！").unwrap(),              // Chinese
        Regex::new(
            r"\[CHAT\] The party was disbanded because all invites expired and the party was empty\.",
        )
        .unwrap(), // English
        Regex::new(r"\[CHAT\] 因组队中没有成员， 且所有邀请均已过期， 组队已被解散。").unwrap(), // Chinese
    ]
}
pub fn get_party_join_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*) joined the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)加入了组队。").unwrap(),        // Chinese
    ]
}
pub fn get_party_leave_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] Kicked (?:\[.*\])?(?:\s)?(.*) because they were offline\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)已断开连接， 被移出你的组队。").unwrap(), // Chinese
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*) has left the party\.").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(.*)离开了组队。").unwrap(),          // Chinese
    ]
}
pub fn get_job_change_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\[CHAT\] The party was transferred to (?:\[.*\])?(?:\s)?(.*) by (?:\[.*\])?(?:\s)?(?:.*)").unwrap(), // English
        Regex::new(r"\[CHAT\] (?:\[.*\])?(?:\s)?(?:.*)将组队移交给了(?:\[.*\])?(?:\s)?(.*)").unwrap(), // Chinese (no "." or "。")
        Regex::new(r"\[CHAT\] The party was transferred to (?:\[.*\])?(?:\s)?(.*) because (?:\[.*\])?(?:\s)?(?:.*) left").unwrap(),
        Regex::new(r"\[CHAT\] 由于(?:\[.*\])?(?:\s)?(?:.*)离开了组队，组队被移交给了(?:\[.*\])?(?:\s)?(.*)").unwrap()
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

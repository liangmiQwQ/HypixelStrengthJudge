use regex::Regex;


pub fn get_useful_party_lines_patterns() -> Vec<Regex> { 
    vec![
      Regex::new(r".*?(.+?) joined the party\.").unwrap(), // English
        Regex::new(r".*?(.+?)加入了组队。").unwrap(),          // Chinese

        Regex::new(r".*?(.+?) has left the party\.").unwrap(), // English
        Regex::new(r".*?(.+?)离开了组队。").unwrap(),          // Chinese

        Regex::new(r".*?You have joined (.+?)'s party!").unwrap(), // English
        Regex::new(r".*?你加入了(.+?)的组队！").unwrap(),         // Chinese

        Regex::new(r".*?You left the party\.").unwrap(),          // English
        Regex::new(r".*?你离开了组队。").unwrap(),                // Chinese

        Regex::new(r".*?(.+?) has been removed from the party\.").unwrap(), // English
        Regex::new(r".*?(.+?)已被移出组队。").unwrap(),                  // Chinese

        Regex::new(r".*?You have been kicked from the party by (.+?)").unwrap(), // English
        Regex::new(r".*?你已被(.+?)踢出组队").unwrap(),                   // Chinese (no "." or "。")

        Regex::new(r".*?Kicked (.+?) because they were offline\.").unwrap(), // English
        Regex::new(r".*?(.+?)已断开连接， 被移出你的组队。").unwrap(),      // Chinese

        Regex::new(r".*?The party was transferred to (.+?) by (.+?)").unwrap(), // English
        Regex::new(r".*?(.+?)将组队移交给了(.+?)").unwrap(),                 // Chinese (no "." or "。")

        Regex::new(r".*?(.+?) has disbanded the party!").unwrap(), // English
        Regex::new(r".*?(.+?)解散了组队！").unwrap(),              // Chinese

        Regex::new(r".*?The party was disbanded because all invites expired and the party was empty\.").unwrap(), // English
        Regex::new(r".*?因组队中没有成员， 且所有邀请均已过期， 组队已被解散。").unwrap(),                            // Chinese
    ]
}

pub fn extract_party_leader(line: &str) -> Option<String> {
    let re = Regex::new(r#"\[CHAT\] Party Leader: (\[.*?\] )?(\w+)"#).unwrap();
    
    if let Some(captures) = re.captures(line) {
        if let Some(name) = captures.get(2) {
            return Some(name.as_str().to_string());
        }
    }
    
    None
}

pub fn extract_party_moderators(line: &str) -> Option<Vec<String>> {
    let re = Regex::new(r#"\[CHAT\] Party Moderators: (.+)"#).unwrap();

    if let Some(captures) = re.captures(line) {
        if let Some(names_str) = captures.get(1) {
            let names = names_str.as_str()
                .split('●')
                .map(|s| s.trim().split(' ').last().unwrap_or("").to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return Some(names);
        }
    }

    // 如果没有匹配，返回 None
    return None
} 


pub fn extract_party_members(line: &str) -> Option<Vec<String>> {
    // 定义正则表达式来匹配 Party Members 名字
    let re = Regex::new(r#"\[CHAT\] Party Members: (.+)"#).unwrap();

    // 尝试匹配正则表达式并提取名字
    if let Some(captures) = re.captures(line) {
        // 提取第一个捕获组（名字列表）
        if let Some(names_str) = captures.get(1) {
            let names = names_str.as_str()
                .split('●')
                .map(|s| s.trim().split(' ').last().unwrap_or("").to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return Some(names);
        }
    }

    // 如果没有匹配，返回 None
    None
}
pub fn reply(line: &str) -> String {
    let s = line.trim();
    if s.is_empty() {
        return "Fine. Be that way!".to_string();
    } else if s == s.to_uppercase() && s != s.to_lowercase() {
        return "Whoa, chill out!".to_string();
    } else if s.chars().last().unwrap() == '?' {
        return "Sure.".to_string();
    } else {
        return "Whatever.".to_string();
    }
}
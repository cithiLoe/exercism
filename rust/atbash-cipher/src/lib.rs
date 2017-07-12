use std::ascii::AsciiExt;

fn is_valid(c: char) -> bool {
    c.is_alphanumeric() && c.is_ascii()
}

fn transform(c: char) -> char {
    if c.is_digit(10) {
        c
    } else {
        ('z' as u8 + 'a' as u8 - c as u8) as char
    }
}

pub fn encode(input: &str) -> String {
    input.to_lowercase()
        .chars()
        .filter(|&c| is_valid(c))
        .map(|c| transform(c))
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(input: &str) -> String {
    input.to_lowercase()
        .chars()
        .filter(|&c| is_valid(c))
        .map(|c| transform(c))
        .collect()
}
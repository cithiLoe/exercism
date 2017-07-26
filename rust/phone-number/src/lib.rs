pub fn number(sequence: &str) -> Option<String> {
    let digits = sequence
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>();
    if digits.len() == 10 {
        return Some(digits);
    } else if digits.len() == 11 {
        if Some('1') == digits.chars().nth(0) {
            return Some(digits[1..].to_string());
        }
    }
    None
}

pub fn area_code(sequence: &str) -> Option<String> {
    number(sequence).map(|digits| digits[..3].to_string())
}

pub fn pretty_print(sequence: &str) -> String {
    number(sequence)
        .map(|digits| format!("({}) {}-{}", &digits[..3], &digits[3..6], &digits[6..]))
        .unwrap_or("invalid".to_string())
}

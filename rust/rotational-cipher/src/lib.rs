pub fn rotate(input: &str, key: u8) -> String {
    if key % 26 == 0 {
        input.to_string()
    } else {
        input.chars()
            .map(|c| transform(c, key))
            .collect()
    }
}

fn transform(ch: char, key: u8) -> char {
    if ch.is_alphabetic() {
        let a = if ch.is_lowercase() { 'a' } else { 'A' } as u8;
        (a + (ch as u8 - a + key) % 26) as char
    } else {
        ch
    }
    // let (a, A, c) = ('a' as u8, 'A' as u8, ch as u8);
    // match ch {
    //     'a'...'z' => (((c - a + key) % 26) + a) as char,
    //     'A'...'Z' => (((c - A + key) % 26) + A) as char,
    //     _ => ch,
    // }
}
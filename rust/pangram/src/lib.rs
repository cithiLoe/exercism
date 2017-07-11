pub fn is_pangram(sentence: &str) -> bool {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|c| sentence.to_lowercase().contains(c))
}
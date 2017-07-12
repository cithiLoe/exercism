use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    input.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        })
}
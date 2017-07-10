pub fn hamming_distance(first: &str, second: &str) -> Result<usize, &'static str> {
    if first.len() != second.len() {
        Err("DNA's lengths differ")
    } else {
        Ok(first.chars().zip(second.chars()).filter(|x| x.0 != x.1).count())
    }
}
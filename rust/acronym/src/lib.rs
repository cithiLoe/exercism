pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            let all_uppercase = word.chars().all(|c| c.is_uppercase());
            let all_lowercase = word.chars().all(|c| c.is_lowercase());
            let same_case = all_uppercase || all_lowercase;
            if same_case {
                word.chars().take(1).collect::<Vec<_>>()
            } else {
                word.chars()
                    .filter(|c| c.is_uppercase())
                    .collect::<Vec<_>>()
            }
        })
        .collect::<String>()
        .to_uppercase()
}

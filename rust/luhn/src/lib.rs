pub fn is_valid(number: &str) -> bool {
    if number.chars().filter(|c| c.is_digit(10)).count() < 2 ||
       number.chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false
    }

    number.chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(index, digit)| 
            if (index + 1) % 2 == 0 { 
                if digit > 4 {
                    digit * 2 - 9 
                } else {
                    digit * 2 
                }
            } else {
                digit
            })
        .sum::<u32>() % 10 == 0
}
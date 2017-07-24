pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        if self.to_string().chars().filter(|c| c.is_digit(10)).count() < 2 ||
           self.to_string()
               .chars()
               .any(|c| !c.is_digit(10) && c != ' ') {
            return false;
        }

        self.to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
            .map(|digit| if digit > 9 { digit - 9 } else { digit })
            .sum::<u32>() % 10 == 0
    }
}

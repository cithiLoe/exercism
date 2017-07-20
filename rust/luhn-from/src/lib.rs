pub struct Luhn {
    digits: String,
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        if self.digits.chars().filter(|c| c.is_digit(10)).count() < 2 ||
            self.digits.chars().any(|c| !c.is_digit(10) && c != ' ') {
            return false
        }

        self.digits.chars()
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
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn { digits: input.to_string() }
    }
}
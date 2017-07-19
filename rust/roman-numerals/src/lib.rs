use std::fmt;

static ROMAN_MAP: [(u32, &'static str); 13] = [(1, "I"),
                                                 (4, "IV"),
                                                 (5, "V"),
                                                 (9, "IX"),
                                                 (10, "X"),
                                                 (40, "XL"),
                                                 (50, "L"),
                                                 (90, "XC"),
                                                 (100, "C"),
                                                 (400, "CD"),
                                                 (500, "D"),
                                                 (900, "CM"),
                                                 (1000, "M")];

pub struct Roman {
    number: u32,
}

impl From<u32> for Roman {
    fn from(number: u32) -> Self {
        Roman::new(number)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.number;
        let mut result = String::new();
        for &(numeric, roman) in ROMAN_MAP.iter().rev() {
            while current >= numeric {
                result.push_str(roman);
                current = current - numeric;
            }
        }
        write!(f, "{}", result)
    }
}

impl Roman {
    fn new(number: u32) -> Roman {
        Roman { number: number }
    }
}
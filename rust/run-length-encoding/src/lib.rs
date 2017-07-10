pub fn encode(input: &str) -> String {
    let mut result = String::new();

    let len = input.len();
    
    let mut run = 0;
    let mut position = 1;
    let mut previous = input.chars().next().unwrap_or_default();
    for current in input.chars() {
        if current == previous {
            if position == len {
                result += (run + 1).to_string().as_str();
                result.push(current);
            } else {
                run += 1;
                position += 1;
            }
        } else {
            if run != 1 {
                result += run.to_string().as_str();
            }
            if run != 0 {
                result.push(previous);
            }
            if position == len {
                result.push(current);
            } else {
                run = 1;
                position += 1;
                previous = current;
            }
        }
    }
    result
}

pub fn decode(input: &str) -> String {
    let mut result = String::new();
    let mut run = 0;
    for current in input.chars() {
        if let Some(digit) = current.to_digit(10) {
            run = run * 10 + digit;
        } else {
            let repeat = if run > 0 { run as usize } else { 1 };
            result += current.to_string().repeat(repeat).as_str();
            run = 0;
        }
    }
    result
}
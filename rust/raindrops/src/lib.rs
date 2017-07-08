pub fn raindrops(number: i32) -> String {
    let mut output = String::new();
    if number % 3 == 0 {
        output += "Pling";
    }
    if number % 5 == 0 {
        output += "Plang"
    }
    if number % 7 == 0 {
        output += "Plong"
    }
    if output.is_empty() {
        output = number.to_string();
    }
    output
}
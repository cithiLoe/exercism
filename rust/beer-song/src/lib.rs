pub fn verse(number: u32) -> String {
    let remaining_number = 
        if number == 0 { 
            99 
        } else { 
            number - 1 
        };

    format!(
        "{} on the wall, {}.\n{}, {} on the wall.\n",
        how_many_bottles(number),
        how_many_bottles(number).to_lowercase(),
        what_action_to_take(number),
        how_many_bottles(remaining_number).to_lowercase()
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut output = verse(start);
    for i in (end..start).rev() {
        output += &"\n".to_string();
        output += &verse(i);
    }
    output
}

fn how_many_bottles(number: u32) -> String {
    match number {
        0 => "No more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        _ => format!("{} bottles of beer", number)
    }
}

fn what_action_to_take(number: u32) -> &'static str {
    match number {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around"
    }
}


// Another solution from taylorzr
// pub fn sing(start: u32, end: u32) -> String {
//     (end..start + 1)
//         .rev()
//         .map(verse)
//         .collect::<Vec<String>>()
//         .join("\n")
// }
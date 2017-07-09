pub fn verse(number: u32) -> String {
    if number == 0 {
        format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if number == 2 {
        format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", number)
    } else if number == 1 {
        format!("{current} bottle of beer on the wall, {current} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", current=number)
    } else {
        format!("{current} bottles of beer on the wall, {current} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", current=number, next=number-1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut output = verse(start);
    let mut current = start;
    while current != end {
        current -= 1;
        output += &"\n".to_string();
        output += &verse(current);
    }
    output
}
pub fn square_of_sum(numbers: u32) -> u32 {
    (1..numbers + 1)
        .sum::<u32>()
        .pow(2)
} 

pub fn sum_of_squares(numbers: u32) -> u32 {
    (1..numbers + 1)
        .map(|x| x * x)
        .sum()
}

pub fn difference(numbers: u32) -> u32{
    square_of_sum(numbers) - sum_of_squares(numbers)
}


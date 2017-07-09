// use std::collections::HashSet;

// pub fn sum_of_multiples(n: u32, divisors: &[u32]) -> u32 {
//     let mut multiplies = HashSet::new();
//     for number in 1..n {
//         for divisor in divisors {
//             if number % divisor == 0 {
//                 multiplies.insert(number);
//             }
//         }
//     }
//     multiplies.iter().sum()
// }

pub fn sum_of_multiples(n: u32, divisors: &[u32]) -> u32 {
    (1..n).filter(|x| divisors.iter().any(|y| x % y == 0)).sum()
}
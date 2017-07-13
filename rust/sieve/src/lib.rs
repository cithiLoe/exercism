pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = vec![true; limit as usize + 1];
    primes[0] = false;
    primes[1] = false;
    let max = (limit as f32).sqrt() as usize + 1;
    for i in 2..max {
        if primes[i] {
            let mut j = i * i;
            while j < limit as usize + 1 {
                primes[j] = false;
                j += i;
            }
        }
    }
    primes.iter()
        .enumerate()
        .filter(|&(_, &value)| value)
        .map(|(index, _)| index as u32)
        .collect()
}
pub fn nth(n: u32) -> Result<u32, ()> {
    if n == 0 {
        Err(())
    } else if n == 1 {
        Ok(2)
    } else {
        let mut i = 1;
        let mut candidate = 1;
        while i < n {
            candidate += 2;
            if is_prime(candidate) {
                i += 1;
            }
        }
        Ok(candidate)
    }
}

fn is_prime(number: u32) -> bool {
    let mut i = 2;
    while i * i < (number + 1) {
        if number % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

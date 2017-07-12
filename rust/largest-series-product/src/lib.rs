pub fn lsp(digits: &str, window: usize) -> Result<u32, ()> {
    if window == 0 {
        Ok(1)
    } else {
        if digits.chars().any(|c| !c.is_digit(10)) ||
            digits.chars().count() < window {
            Err(())
        } else {
            let products = digits.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
                .windows(window)
                .map(|x| x.iter().product())
                .collect::<Vec<u32>>();
            
            if let Some(&x) = products.iter().max() {
                Ok(x)
            } else {
                Err(())
            }
        }
    }
}
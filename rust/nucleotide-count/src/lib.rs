use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, ()>{
    let valid = |x: char| { NUCLEOTIDES.contains(x) };
    if valid(nucleotide) && dna.chars().all(valid) {
        Ok(dna.chars().filter(|&c| c == nucleotide).count())
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut nucleotides = NUCLEOTIDES.chars()
        .map(|c| (c, 0))
        .collect::<HashMap<char, usize>>();
    for c in dna.chars() {
        if let Some(counter) = nucleotides.get_mut(&c) {
            *counter += 1;
        } else {
            return Err(())
        }
    }
    Ok(nucleotides)
}
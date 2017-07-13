#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    nucleotides: String,
}

pub struct DeoxyribonucleicAcid {
    nucleotides: String,
}

impl RibonucleicAcid {
    pub fn new(nucleotides: &str) -> RibonucleicAcid {
        RibonucleicAcid { nucleotides: nucleotides.to_string() }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, ()> {
        let result = self.nucleotides
            .chars()
            .filter_map(transcript)
            .collect::<String>();
        if result.len() == self.nucleotides.len() {
            Ok(RibonucleicAcid::new(result.as_str()))
        } else {
            Err(())
        }
    }
}

fn transcript(c: char) -> Option<char> {
    match c {
        'G' => Some('C'),
        'C' => Some('G'),
        'A' => Some('U'),
        'T' => Some('A'),
        _ => None,
    }
}

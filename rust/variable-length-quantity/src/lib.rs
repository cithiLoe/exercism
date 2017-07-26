const EIGHTBITMASK: u32 = 0x80;
const SEVENBITMASK: u32 = 0x7f;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&v| to_bytes_single(v)).collect()
}

fn to_bytes_single(value: u32) -> Vec<u8> {
    let mut val = value;
    let mut bytes = vec![(val & SEVENBITMASK) as u8];

    val >>= 7;

    while val > 0 {
        bytes.push((val & SEVENBITMASK | EIGHTBITMASK) as u8);
        val >>= 7;
    }

    bytes.reverse();
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut res = Vec::new();
    let mut n = 0;

    for (index, byte) in bytes.iter().enumerate() {

        if (n & 0xfe_00_00_00) > 0 {
            return Err("Would overflow");
        }

        n = (n << 7) | ((*byte as u32) & SEVENBITMASK);

        if (*byte as u32) & EIGHTBITMASK == 0 {
            res.push(n);
            n = 0;
        } else if index == bytes.len() - 1 {
            return Err("Incomplete byte sequence");
        }
    }

    Ok(res)
}

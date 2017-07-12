use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input.iter()
        .flat_map(|(&k, v)| 
            v.iter().map(move |c| (c.to_ascii_lowercase(), k))
        ).collect()
}
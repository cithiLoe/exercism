#[derive(PartialEq, Debug)]
pub enum Comparison {
    Sublist,
    Superlist,
    Equal,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if contains(a, b) {
        Comparison::Superlist
    } else if contains(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        for i in 0..(a.len() - b.len() + 1) {
            if a[i..].starts_with(b) {
                return true;
            }
        }
    }
    false
}
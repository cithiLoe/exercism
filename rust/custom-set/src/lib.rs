// "How it works internally doesn't matter" - readme
#[derive(Debug)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.difference(other).is_empty() && other.difference(self).is_empty()
    }
}

impl<T: Ord + Clone> CustomSet<T> {
    pub fn new(input: Vec<T>) -> Self {
        let mut set = CustomSet { elements: input };
        set.elements.sort();
        set.elements.dedup();
        set
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn contains(&self, n: &T) -> bool {
        self.elements.contains(n)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|x| other.contains(x))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.elements.iter().any(|x| other.contains(x))
    }

    pub fn add(&mut self, n: T) {
        if !self.elements.contains(&n) {
            self.elements.push(n);
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(self.elements
                           .iter()
                           .cloned()
                           .filter(|x| other.contains(x))
                           .collect())
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(self.elements
                           .iter()
                           .cloned()
                           .filter(|x| !other.contains(x))
                           .collect())
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new(self.elements
                           .iter()
                           .cloned()
                           .chain(other.elements.iter().cloned())
                           .collect())
    }
}

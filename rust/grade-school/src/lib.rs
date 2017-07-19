use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // self.grades.entry(grade).or_insert(Vec::new()).push(student.to_string());
        let mut entry = self.grades.entry(grade).or_insert(Vec::new());
        entry.push(student.to_string());
        entry.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut gr = self.grades.keys().cloned().collect::<Vec<u32>>();
        gr.sort();
        gr
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| v.iter().cloned().collect())
    }
}
// use std::collections::{BTreeMap, BTreeSet};

// pub struct School {
//     grades: BTreeMap<u32, BTreeSet<String>>
// }

// impl School {
//     pub fn new() -> School {
//         School { grades: BTreeMap::new() }
//     }

//     pub fn add(&mut self, grade: u32, student: &str) {
//         self.grades
//             .entry(grade)
//             .or_insert(BTreeSet::new())
//             .insert(student.to_string());
//     }

//     pub fn grades(&self) -> Vec<u32> {
//         self.grades.keys().cloned().collect::<Vec<u32>>()
//     }

//     // If grade returned an `Option<&Vec<String>>`,
//     // the internal implementation would be forced to keep a `Vec<String>` to lend out.
//     // By returning an owned vector instead,
//     // the internal implementation is free to use whatever it chooses.
//     pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
//         self.grades.get(&grade).map(|k| k.iter().cloned().collect())
//     }
// }

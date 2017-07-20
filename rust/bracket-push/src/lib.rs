use std::collections::HashMap;

#[derive(Debug)]
pub struct Brackets {
    brackets: Vec<char>,
}

impl Brackets {
    fn new(input: String) -> Self {
        let brackets = input
            .chars()
            .filter(|&c| "()[]{}".chars().any(|s| s == c))
            .collect::<Vec<char>>();
        Brackets { brackets: brackets }
    }

    pub fn are_balanced(self) -> bool {
        let pairs = vec![(']', '['), ('}', '{'), (')', '(')]
            .into_iter()
            .collect::<HashMap<char, char>>();
        let mut stack = Vec::new();
        for bracket in self.brackets.iter() {
            if pairs.values().find(|&v| v == bracket).is_some() {
                stack.push(bracket)
            } else if pairs.contains_key(&bracket) {
                if pairs.get(&bracket) != stack.pop() {
                    return false;
                }
            }
        }
        stack.is_empty()
        // let mut stack = Vec::new();
        // for bracket in self.brackets.into_iter() {
        //     match bracket {
        //         '(' => stack.push(')'),
        //         '[' => stack.push(']'),
        //         '{' => stack.push('}'),
        //         ')' | ']' | '}' => if Some(bracket) != stack.pop() {
        //             return false;
        //         },
        //         _ => (),
        //     }
        // }
        // stack.is_empty()
    }
}

impl<'a> From<&'a str> for Brackets {
    fn from(input: &str) -> Self {
        Brackets::new(input.to_string())
    }
}

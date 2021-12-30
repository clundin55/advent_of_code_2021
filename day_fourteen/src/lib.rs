use regex::Regex;
use std::collections::HashMap;

pub const DAY_FOURTEEN_INPUT: &str = "puzzle_input.txt";
pub const DAY_FOURTEEN_EXAMPLE_INPUT: &str = "example_input.txt";

fn polymer_counter(polymer: &str) -> HashMap<String, u32> {
    let mut counter = HashMap::new();
    for c in polymer.chars() {
        let count = counter.entry(c.to_string()).or_insert(1);
        *count += 1;
    }
    counter
}

pub fn mutate_polymer(start: &str, rules: HashMap<&str, &str>, iterations: u32) -> Option<u32> {
    let mut current_polymer = start.to_string();
    for _ in 0..iterations {
        let mut new_insertions: Vec<(usize, &str)> = Vec::new();
        for idx in 0..current_polymer.len() - 1 {
            let pair = current_polymer.get(idx..idx + 2).unwrap();
            if let Some(new_polymer) = rules.get(pair) {
                new_insertions.push((idx + 1, new_polymer));
            }
        }
        let mut index_drift = 0;
        for insertion in new_insertions {
            current_polymer.insert_str(insertion.0 + index_drift, &insertion.1);
            index_drift += 1;
        }
    }

    let count = polymer_counter(&current_polymer);
    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();

    Some(*max - *min)
}

pub fn get_instructions(text: &str) -> Option<(&str, HashMap<&str, &str>)> {
    let rule_regex = Regex::new(r"(..) -> (.)").unwrap();
    let mut polymer_rules: HashMap<&str, &str> = HashMap::new();

    let start = text.lines().nth(0).unwrap();

    for line in text.lines() {
        if rule_regex.is_match(line) {
            let caps = rule_regex.captures(line).unwrap();
            polymer_rules.insert(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        }
    }
    Some((start, polymer_rules))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn example_polymers() {
        let mut file = File::open(DAY_FOURTEEN_EXAMPLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        if let Some((start, rules)) = get_instructions(&contents) {
            assert_eq!(mutate_polymer(&start, rules, 10), Some(1588));
        }
    }

    #[test]
    fn polymers() {
        let mut file = File::open(DAY_FOURTEEN_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        if let Some((start, rules)) = get_instructions(&contents) {
            assert_eq!(mutate_polymer(&start, rules, 10), Some(2745));
        }
    }
}

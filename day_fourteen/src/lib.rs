use regex::Regex;
use std::collections::HashMap;

fn mutate_polymer(start: String, rules: HashMap<String,String>, iterations: u32) -> Option<u32> {
    let mut current_polymer = start.clone();
    for _ in 0..iterations {
        let mut new_insertions: Vec<(usize, String)> = Vec::new();
        for idx in 0..current_polymer.len() - 1 {
            let pair = current_polymer.get(idx..idx+1).unwrap();
            if let Some(new_polymer) = rules.get(pair) {
                new_insertions.push((idx, new_polymer.to_string()));
            }
        }
        for insertion in new_insertions {
            current_polymer.insert_str(insertion.0, &insertion.1);
        }
    }


    None
}

fn get_instructions(text: &str) -> Option<(String,HashMap<String,String>)> {
    let rule_regex = Regex::new(r"(..) -> (.)").unwrap();
    let mut polymer_rules: HashMap<String,String> = HashMap::new();

    let mut start = text.lines().nth(0).unwrap();

    for line in text.lines() {
        if rule_regex.is_match(line) {
            let caps = rule_regex.captures(line).unwrap();
            polymer_rules.insert(
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().to_string()
            );
        }
    }
    Some((start.to_string(),polymer_rules))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const DAY_FOURTEEN_INPUT: &str = "puzzle_input.txt";
    const DAY_FOURTEEN_EXAMPLE_INPUT: &str = "example_input.txt";

    #[test]
    fn example_fold_manual() {
        let mut file = File::open(DAY_FOURTEEN_EXAMPLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        if let Some((start, rules)) = get_instructions(&contents) {
            dbg!(&start);
            dbg!(&rules);
            assert_eq!(1, 17);
        }
    }
}


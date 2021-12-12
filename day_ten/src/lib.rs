use std::collections::{HashMap, HashSet};

fn is_opener(c: &char) -> bool {
    HashSet::from(['[', '{', '<', '(']).contains(c)
}

fn expected_delimiter(c: &char) -> Option<char> {
    let limiter_delimiter_association =
        HashMap::from([('[', ']'), ('{', '}'), ('<', '>'), ('(', ')')]);
    if let Some(delimiter) = limiter_delimiter_association.get(&c) {
        return Some(delimiter.clone());
    }
    None
}

fn score_error(c: &char) -> Option<u64> {
    let error_scores = HashMap::from([(']', 57), ('}', 1197), ('>', 25137), (')', 3)]);
    if let Some(score) = error_scores.get(c) {
        return Some(score.clone());
    }
    None
}

fn fix_nav_system(input: &str) -> Option<u64> {
    let mut first_error: Vec<char> = Vec::new();

    for line in input.lines() {
        let mut syntax_tracker = Vec::new();
        for c in line.chars() {
            if is_opener(&c) {
                syntax_tracker.push(c);
                continue;
            }

            let opener = syntax_tracker.pop()?;
            let expected_delimiter = expected_delimiter(&opener)?;

            if expected_delimiter != c {
                first_error.push(c);
                break;
            }
        }
    }

    first_error
        .iter()
        .map(|c| score_error(c))
        .filter(|r| r.is_some())
        .sum()
}

fn score_incomplete_line(c: &char) -> Option<u64> {
    let incomplte_line_scores = HashMap::from([('[', 2), ('{', 3), ('<', 4), ('(', 1)]);
    if let Some(score) = incomplte_line_scores.get(c) {
        return Some(score.clone());
    }
    None
}

fn fix_nav_system_incomplete_lines(input: &str) -> Option<u64> {
    let mut incomplete_lines: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut syntax_tracker = Vec::new();
        let mut no_errors = true;
        for c in line.chars() {
            if is_opener(&c) {
                syntax_tracker.push(c);
                continue;
            }

            let opener = syntax_tracker.pop()?;
            let expected_delimiter = expected_delimiter(&opener)?;

            if expected_delimiter != c {
                no_errors = false;
                break;
            }
        }
        if no_errors {
            incomplete_lines.push(syntax_tracker);
        }
    }

    let scores = incomplete_lines.iter().map(|l| {
        l.iter()
            .rev()
            .fold(0, |acc, c| match score_incomplete_line(&c) {
                Some(score) => score + acc * 5,
                _ => 0,
            })
    });
    let mut scores: Vec<u64> = scores.collect();
    scores.sort();
    Some(scores[scores.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const EXAMPLE_INPUT: &str = "example_input.txt";
    const PUZZLE_INPUT: &str = "puzzle_input.txt";

    #[test]
    fn example_fix_nav() {
        let mut file = File::open(EXAMPLE_INPUT.to_string()).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(Some(26397), fix_nav_system(&input));
    }

    #[test]
    fn example_fix_nav_system_incomplete_lines() {
        let mut file = File::open(EXAMPLE_INPUT.to_string()).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(Some(288957), fix_nav_system_incomplete_lines(&input));
    }

    #[test]
    fn fix_nav() {
        let mut file = File::open(PUZZLE_INPUT.to_string()).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(Some(271245), fix_nav_system(&input));
    }

    #[test]
    fn fix_nav_incomplete_lines() {
        let mut file = File::open(PUZZLE_INPUT.to_string()).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(Some(1685293086), fix_nav_system_incomplete_lines(&input));
    }
}

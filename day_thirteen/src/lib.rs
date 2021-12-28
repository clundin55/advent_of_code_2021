use regex::Regex;

#[derive(Debug)]
enum FoldDirection {
    Vertical(u32),
    Horizontal(u32),
}

impl FoldDirection {
    fn from_str(input: &str) -> Option<Self> {
        let fold = Regex::new(r"fold along (.)=(\d+)").unwrap();
        if let Some(cap) = fold.captures(input) {
            if let Some(direction) = cap.get(1) {
                let direction = direction.as_str();
                if direction.eq("x") {
                    return Some(FoldDirection::Horizontal(
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                    ));
                }

                if direction.eq("y") {
                    return Some(FoldDirection::Vertical(
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                    ));
                }
            }
        }
        None
    }
}

type Point = (usize, usize);

fn get_instructions(text: &str) -> Option<(Vec<Point>, Vec<FoldDirection>)> {
    let coord = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut coords: Vec<Point> = Vec::new();
    let mut folds: Vec<FoldDirection> = Vec::new();

    for line in text.lines() {
        if coord.is_match(line) {
            let caps = coord.captures(line).unwrap();
            coords.push((
                caps.get(1).unwrap().as_str().to_string().parse().unwrap(),
                caps.get(2).unwrap().as_str().to_string().parse().unwrap(),
            ));
        }

        if let Some(fold) = FoldDirection::from_str(line) {
            folds.push(fold);
        }
    }
    Some((coords, folds))
}

fn fold_manual(manual: Vec<Point>, folds: Vec<FoldDirection>) -> u32 {
    let mut working_manual = manual.clone();
    for fold in folds {
        match fold {
            FoldDirection::Vertical(location) => {
                working_manual.rotate_left

            },
            FoldDirection::Horizontal(location) => {
                for line in working_manual {

                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const DAY_THIRTEEN_INPUT: &str = "puzzle_input.txt";
    const DAY_THIRTEEN_EXAMPLE_INPUT: &str = "example_input.txt";

    #[test]
    fn example_fold_manual() {
        let mut file = File::open(DAY_THIRTEEN_EXAMPLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        if let Some((coords, folds)) = get_instructions(&contents) {
            dbg!(&coords);
            dbg!(&folds);
            let result = fold_manual(coords, folds);
            assert_eq!(result, 17);
        }
    }
}

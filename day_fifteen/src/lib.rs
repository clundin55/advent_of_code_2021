use std::collections::VecDeque;

pub const DAY_FIFTEEN_INPUT: &str = "puzzle_input.txt";
pub const DAY_FIFTEEN_EXAMPLE_INPUT: &str = "example_input.txt";

pub fn find_low_risk_path(cave_risk_map: &Vec<Vec<u32>>) -> Option<u32> {
    let mut paths = VecDeque::new();
    let mut path_values = Vec::new();

    let end = (cave_risk_map.len() - 1, cave_risk_map[0].len() - 1);
    let mut garbage_collector = 0;

    paths.push_back(((0, 0), 0));

    while !paths.is_empty() {
        if let Some((pos, val)) = paths.pop_front() {
            if pos.0 == end.0 && pos.1 == end.1 {
                path_values.push(val);
            } else {
                if pos.0 < cave_risk_map.len() - 1 {
                    paths.push_back(((pos.0 + 1, pos.1), val + cave_risk_map[pos.0 + 1][pos.1]));
                }
                if pos.1 < cave_risk_map[pos.0].len() - 1 {
                    paths.push_back(((pos.0, pos.1 + 1), val + cave_risk_map[pos.0][pos.1 + 1]));
                }
            }
        }
        //dbg!(pos,val);
        //dbg!(paths);
    }

    path_values.into_iter().min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn example_risk_path() {
        let input = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];
        assert_eq!(Some(40), find_low_risk_path(&input))
    }

    #[test]
    fn risk_path() {
        let mut file = File::open(DAY_FIFTEEN_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let input: Vec<Vec<u32>> = contents
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        assert_eq!(Some(40), find_low_risk_path(&input))
    }
}

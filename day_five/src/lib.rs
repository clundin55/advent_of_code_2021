use std::cmp::{max, min};
use std::collections::HashMap;

const DAY_FIVE_INPUT: &str = "puzzle_input.txt";
type Point = (i32, i32);

fn generate_path(p1: Point, p2: Point) -> Vec<Point> {
    if p1.0 == p2.0 {
        let start = min(p1.1, p2.1);
        let end = max(p1.1, p2.1);
        let path = start..=end;
        return path.map(|y| (p1.0, y)).collect();
    }

    if p1.1 == p2.1 {
        let start = min(p1.0, p2.0);
        let end = max(p1.0, p2.0);
        let path = start..=end;
        return path.map(|x| (x, p1.1)).collect();
    }

    Vec::new()
}

fn generate_path_with_diagonal(p1: Point, p2: Point) -> Vec<Point> {
    let x_start = min(p1.0, p2.0);
    let x_end = max(p1.0, p2.0);

    let y_start = min(p1.1, p2.1);
    let y_end = max(p1.1, p2.1);

    let x_path = x_start..=x_end;
    let y_path = y_start..=y_end;

    if p1.0 == p2.0 {
        return y_path.map(|y| (p1.0, y)).collect();
    }

    if p1.1 == p2.1 {
        return x_path.map(|x| (x, p1.1)).collect();
    }

    if (p1.0 - p2.0).abs() == (p1.1 - p2.1).abs() {
        if p1.0 > p2.0 && p1.1 > p2.1 {
            return x_path.rev().zip(y_path.rev()).collect();
        } else if p1.1 > p2.1 {
            return x_path.zip(y_path.rev()).collect();
        } else if p1.0 > p2.0 {
            return x_path.rev().zip(y_path).collect();
        } else {
            return x_path.zip(y_path).collect();
        }
    }

    Vec::new()
}

fn n_lines_overlap(lines: Vec<(Point, Point)>, n: u32) -> usize {
    let mut point_counter: HashMap<Point, u32> = HashMap::new();
    for (p1, p2) in lines {
        let path = generate_path(p1, p2);
        for point in path {
            let line_intersections = point_counter.entry(point).or_insert(0);
            *line_intersections += 1;
        }
    }
    point_counter.values().filter(|&&v| v >= n).count()
}

fn n_lines_overlap_with_diagonal(lines: Vec<(Point, Point)>, n: u32) -> usize {
    let mut point_counter: HashMap<Point, u32> = HashMap::new();
    for (p1, p2) in lines {
        let path = generate_path_with_diagonal(p1, p2);
        for point in path {
            let line_intersections = point_counter.entry(point).or_insert(0);
            *line_intersections += 1;
        }
    }
    point_counter.values().filter(|&&v| v >= n).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn generate_path_vertical() {
        let p1 = (0, 1);
        let p2 = (0, 2);
        let expected = vec![(0, 1), (0, 2)];
        assert_eq!(expected, generate_path(p1, p2));
    }

    #[test]
    fn generate_path_diagonal() {
        let p1 = (2, 0);
        let p2 = (0, 2);
        let expected = vec![(2, 0), (1, 1), (0, 2)];
        assert_eq!(expected, generate_path_with_diagonal(p1, p2));
    }

    #[test]
    fn example_hydro_vents() {
        let lines = Vec::from([
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ]);

        assert_eq!(5, n_lines_overlap(lines, 2));
    }

    #[test]
    fn hydro_vents() {
        let mut file = File::open(DAY_FIVE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let lines: Vec<(Point, Point)> = contents
            .lines()
            .map(|s| s.split_once("->").unwrap())
            .map(|s| {
                let p1 = s.0.split_once(",").unwrap();
                let p1 = (p1.0.trim().parse().unwrap(), p1.1.trim().parse().unwrap());
                let p2 = s.1.split_once(",").unwrap();
                let p2 = (p2.0.trim().parse().unwrap(), p2.1.trim().parse().unwrap());
                (p1, p2)
            })
            .collect();

        assert_eq!(7473, n_lines_overlap(lines, 2));
    }

    #[test]
    fn example_hydro_vents_with_diagonal() {
        let lines = Vec::from([
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ]);

        assert_eq!(12, n_lines_overlap_with_diagonal(lines, 2));
    }

    #[test]
    fn hydro_vents_diagonal() {
        let mut file = File::open(DAY_FIVE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let lines: Vec<(Point, Point)> = contents
            .lines()
            .map(|s| s.split_once("->").unwrap())
            .map(|s| {
                let p1 = s.0.split_once(",").unwrap();
                let p1 = (p1.0.trim().parse().unwrap(), p1.1.trim().parse().unwrap());
                let p2 = s.1.split_once(",").unwrap();
                let p2 = (p2.0.trim().parse().unwrap(), p2.1.trim().parse().unwrap());
                (p1, p2)
            })
            .collect();

        assert_eq!(24164, n_lines_overlap_with_diagonal(lines, 2));
    }
}

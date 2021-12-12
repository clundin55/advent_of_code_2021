use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

fn find_neighbors(pos: Point, n: usize, m: usize) -> Vec<Point> {
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    let neighbors = vec![
        ((x - 1) as usize, y as usize),
        ((x + 1) as usize, y as usize),
        (x as usize, (y - 1) as usize),
        (x as usize, (y + 1) as usize),
    ];
    neighbors
        .into_iter()
        .filter(|&(x, y)| x < n && y < m)
        .collect()
}

fn find_smoke_flows<const N: usize, const M: usize>(input: &[[u32; M]; N]) -> Option<u32> {
    let mut low_points = Vec::new();
    for x in 0..N {
        for y in 0..M {
            let neighbors = find_neighbors((x, y), N, M);
            let low_point = input[x][y];
            if low_point < neighbors.iter().map(|&(x, y)| input[x][y]).min()? {
                low_points.push(low_point);
            }
        }
    }
    Some(low_points.iter().map(|x| x + 1).sum())
}

fn find_smoke_flows_basins<const N: usize, const M: usize>(input: &[[u32; M]; N]) -> Option<u32> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut basin: VecDeque<Point> = VecDeque::new();
    let mut basin_sizes = Vec::new();

    for x in 0..N {
        for y in 0..M {
            if visited.contains(&(x, y)) {
                continue;
            }

            if input[x][y] == 9 {
                visited.insert((x, y));
                continue;
            }

            let mut basin_size = 0;
            basin.push_back((x, y));

            while !basin.is_empty() {
                if let Some((i, j)) = basin.pop_front() {
                    if visited.contains(&(i, j)) {
                        continue;
                    }

                    visited.insert((i, j));
                    basin_size += 1;

                    let neighbors = find_neighbors((i, j), N, M);
                    for neighbor in neighbors.iter().filter(|&&(x, y)| input[x][y] != 9) {
                        basin.push_back(*neighbor);
                    }
                }
            }
            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    Some(basin_sizes[0..3].iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const PUZZLE_INPUT: &str = "puzzle_input.txt";

    #[test]
    fn example_smoke_flows() {
        let low_points = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(Some(15), find_smoke_flows(&low_points));
    }

    #[test]
    fn example_smoke_flows_basins() {
        let low_points = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(Some(1134), find_smoke_flows_basins(&low_points));
    }

    #[test]
    fn smoke_flows() {
        let mut file = File::open(PUZZLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();
        let mut input = [[0; 100]; 100];

        file.read_to_string(&mut contents).unwrap();
        let low_points: Vec<Vec<u32>> = contents
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        for x in 0..100 {
            for y in 0..100 {
                input[x][y] = low_points[x][y];
            }
        }

        assert_eq!(Some(522), find_smoke_flows(&input));
    }

    #[test]
    fn smoke_flows_basins() {
        let mut file = File::open(PUZZLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();
        let mut input = [[0; 100]; 100];

        file.read_to_string(&mut contents).unwrap();
        let low_points: Vec<Vec<u32>> = contents
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        for x in 0..100 {
            for y in 0..100 {
                input[x][y] = low_points[x][y];
            }
        }

        assert_eq!(Some(916688), find_smoke_flows_basins(&input));
    }
}

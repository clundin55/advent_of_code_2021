use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

fn get_neighbors(pos: &Point, n: usize, m: usize) -> Vec<Point> {
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    let neighbors = vec![
        ((x - 1) as usize, y as usize),
        ((x + 1) as usize, y as usize),
        (x as usize, (y - 1) as usize),
        (x as usize, (y + 1) as usize),
        ((x + 1) as usize, (y - 1) as usize),
        ((x - 1) as usize, (y + 1) as usize),
        ((x + 1) as usize, (y + 1) as usize),
        ((x - 1) as usize, (y - 1) as usize),
    ];
    neighbors
        .into_iter()
        .filter(|&(x, y)| x < n && y < m)
        .collect()
}

/// 1. Each octopus energy increases by 1.
/// 1. Each octopus energy greater than 9 flashes.
/// 1. All adjacent octopus get 1 energy from flashing
/// 1. Octupus can flash at most once
/// 1. Octupus that flash return to 0
fn octopus_flashes(octopus_energies: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashed: HashSet<Point> = HashSet::new();
    let mut flashes = VecDeque::new();
    let mut flash_count = 0;

    for x in 0..octopus_energies.len() {
        for y in 0..octopus_energies[x].len() {
            octopus_energies[x][y] += 1;
        }
    }

    for x in 0..octopus_energies.len() {
        for y in 0..octopus_energies[x].len() {
            flashes.push_back((x, y));

            while !flashes.is_empty() {
                if let Some((i, j)) = flashes.pop_front() {
                    let octopus = octopus_energies[i][j];
                    if octopus > 9 && !flashed.contains(&(i, j)) {
                        flashed.insert((i, j));
                        flash_count += 1;

                        for neighbor in get_neighbors(
                            &(i, j),
                            octopus_energies.len(),
                            octopus_energies[x].len(),
                        ) {
                            octopus_energies[neighbor.0][neighbor.1] += 1;
                            flashes.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    for flash in flashed {
        octopus_energies[flash.0][flash.1] = 0;
    }

    flash_count
}

fn all_octopus_flashed(octopus_energies: &mut Vec<Vec<u32>>) -> bool {
    let mut flashed: HashSet<Point> = HashSet::new();
    let mut flashes = VecDeque::new();
    let mut flash_count = 0;

    for x in 0..octopus_energies.len() {
        for y in 0..octopus_energies[x].len() {
            octopus_energies[x][y] += 1;
        }
    }

    for x in 0..octopus_energies.len() {
        for y in 0..octopus_energies[x].len() {
            flashes.push_back((x, y));

            while !flashes.is_empty() {
                if let Some((i, j)) = flashes.pop_front() {
                    let octopus = octopus_energies[i][j];
                    if octopus > 9 && !flashed.contains(&(i, j)) {
                        flashed.insert((i, j));
                        flash_count += 1;

                        for neighbor in get_neighbors(
                            &(i, j),
                            octopus_energies.len(),
                            octopus_energies[x].len(),
                        ) {
                            octopus_energies[neighbor.0][neighbor.1] += 1;
                            flashes.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    for flash in &flashed {
        octopus_energies[flash.0][flash.1] = 0;
    }

    flashed.len() == octopus_energies.len() * octopus_energies[0].len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const EXAMPLE_INPUT: &str = "example_input.txt";
    const PUZZLE_INPUT: &str = "puzzle_input.txt";

    #[test]
    fn example_octupus_flashes() {
        let mut octopy = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let mut flashes = 0;
        for i in 0..10 {
            flashes += octopus_flashes(&mut octopy);
        }

        assert_eq!(204, flashes);

        for _ in 0..90 {
            flashes += octopus_flashes(&mut octopy);
        }

        assert_eq!(1656, flashes);
    }

    #[test]
    fn example_all_octupus_flashed() {
        let mut octopy = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        for _ in 0..194 {
            all_octopus_flashed(&mut octopy);
        }

        assert_eq!(true, all_octopus_flashed(&mut octopy));
    }

    #[test]
    fn octopus_flash() {
        let mut octopy = vec![
            vec![5, 6, 5, 1, 3, 4, 1, 4, 5, 2],
            vec![1, 3, 8, 1, 5, 4, 1, 2, 5, 2],
            vec![1, 8, 7, 8, 4, 3, 5, 2, 2, 4],
            vec![6, 8, 1, 4, 8, 3, 1, 5, 3, 5],
            vec![3, 8, 8, 3, 5, 4, 7, 3, 8, 3],
            vec![6, 4, 7, 3, 5, 4, 8, 4, 6, 4],
            vec![1, 8, 8, 5, 8, 3, 3, 6, 5, 8],
            vec![3, 7, 3, 2, 5, 8, 4, 7, 5, 2],
            vec![1, 8, 8, 1, 5, 4, 6, 1, 2, 8],
            vec![5, 1, 2, 1, 7, 1, 7, 7, 7, 6],
        ];

        let mut flashes = 0;

        for _ in 0..100 {
            flashes += octopus_flashes(&mut octopy);
        }

        assert_eq!(1625, flashes);
    }

    #[test]
    fn all_octopus_flash() {
        let mut octopy = vec![
            vec![5, 6, 5, 1, 3, 4, 1, 4, 5, 2],
            vec![1, 3, 8, 1, 5, 4, 1, 2, 5, 2],
            vec![1, 8, 7, 8, 4, 3, 5, 2, 2, 4],
            vec![6, 8, 1, 4, 8, 3, 1, 5, 3, 5],
            vec![3, 8, 8, 3, 5, 4, 7, 3, 8, 3],
            vec![6, 4, 7, 3, 5, 4, 8, 4, 6, 4],
            vec![1, 8, 8, 5, 8, 3, 3, 6, 5, 8],
            vec![3, 7, 3, 2, 5, 8, 4, 7, 5, 2],
            vec![1, 8, 8, 1, 5, 4, 6, 1, 2, 8],
            vec![5, 1, 2, 1, 7, 1, 7, 7, 7, 6],
        ];

        let mut count = 0;
        loop {
            count += 1;
            if all_octopus_flashed(&mut octopy) {
                break;
            }
        }

        assert_eq!(244, count);
    }
}

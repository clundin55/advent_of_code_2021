use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Point = (u32, u32);

pub const DAY_THREE_INPUT: &str = "puzzle_input.txt";

#[derive(Debug, Clone)]
pub struct Board<const M: usize, const N: usize> {
    placements: HashMap<u32, Point>,
    copy_of_board: [[u32; M]; N],
    squares_placed: HashSet<Point>,
    last_draw: u32,
}

impl<const M: usize, const N: usize> Board<M, N> {
    fn build_board_from_array(board: &[[u32; M]; N]) -> Self {
        let n = board.len();
        let m = board[0].len();
        let mut new_board: HashMap<u32, Point> = HashMap::new();

        for (x, y) in (0..n).cartesian_product(0..m) {
            new_board.insert(board[x][y], (x as u32, y as u32));
        }

        Board {
            placements: new_board,
            copy_of_board: *board,
            squares_placed: HashSet::new(),
            last_draw: 0,
        }
    }

    fn apply_draw(&mut self, draw: u32) {
        if let Some((x, y)) = self.placements.get(&draw) {
            self.squares_placed.insert((*x, *y));
            self.last_draw = draw;
        }
    }

    fn sum_of_all_unmarked_numbers(&self) -> u32 {
        let mut unmarked_squares = 0;
        for x in 0..self.copy_of_board.len() {
            for y in 0..self.copy_of_board[x].len() {
                if !self.squares_placed.contains(&(x as u32, y as u32)) {
                    unmarked_squares += self.copy_of_board[x][y];
                }
            }
        }
        unmarked_squares
    }

    fn last_draw(&self) -> u32 {
        self.last_draw
    }

    fn can_win(&self) -> bool {
        let mut visited: HashSet<Point> = HashSet::new();

        let n = self.copy_of_board.len();
        let m = self.copy_of_board[0].len();

        for placement in &self.squares_placed {
            if visited.contains(placement) {
                continue;
            }
            visited.insert(*placement);

            let x = placement.0;
            let y = placement.1;

            let horizontal_line = 0..n;
            let vertical_line = 0..m;

            let mut horizontal_in_a_row = 0;
            for i in horizontal_line {
                if let Some(draw) = self.squares_placed.get(&(x, i as u32)) {
                    visited.insert(*draw);
                    horizontal_in_a_row += 1;
                }
            }
            if horizontal_in_a_row == n {
                return true;
            }

            let mut vertical_in_a_row = 0;
            for i in vertical_line {
                if let Some(draw) = self.squares_placed.get(&(i as u32, y)) {
                    visited.insert(*draw);
                    vertical_in_a_row += 1;
                }
            }

            if vertical_in_a_row == n {
                return true;
            }

            visited.insert(*placement);
        }
        false
    }
}

pub fn play_bingo<const M: usize, const N: usize>(
    boards: &mut Vec<Board<M, N>>,
    draws: &[u32],
) -> u32 {
    for draw in draws {
        for board in &mut *boards {
            board.apply_draw(*draw);
            if board.can_win() {
                return board.last_draw() * board.sum_of_all_unmarked_numbers();
            }
        }
    }
    0
}

pub fn play_bingo_win_last<const M: usize, const N: usize>(
    boards: &mut Vec<Board<M, N>>,
    draws: &[u32],
) -> u32 {
    let mut winners: Vec<usize> = Vec::new();
    for draw in draws {
        if winners.len() == boards.len() {
            break;
        }
        for (idx, board) in boards.iter_mut().enumerate() {
            if !winners.contains(&idx) && (board.can_win()) {
                winners.push(idx);
            } else {
                board.apply_draw(*draw);
            }
        }
    }
    let last_winner = &boards[winners.pop().unwrap()];
    last_winner.last_draw() * last_winner.sum_of_all_unmarked_numbers()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_example_bingo() {
        let draws = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = [
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ],
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ],
        ];
        let mut bingo_boards: Vec<Board<5, 5>> = Vec::new();

        for board in boards {
            bingo_boards.push(Board::build_board_from_array(&board));
        }
        assert_eq!(4512, play_bingo(&mut bingo_boards, &draws));
    }

    #[test]
    fn test_bingo() {
        let mut bingo_boards: Vec<Board<5, 5>> = Vec::new();
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let mut board = [[0; 5]; 5];
        let mut first = true;
        let mut draws: Vec<u32> = Vec::new();
        let mut board_count = 0;
        for line in contents.lines() {
            if first {
                draws = line.split(',').map(|c| c.parse::<u32>().unwrap()).collect();
                first = false;
                continue;
            }

            if line.is_empty() {
                bingo_boards.push(Board::build_board_from_array(&board));
                board_count = 0;
                continue;
            }

            let board_line: Vec<u32> = line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect();
            for (column, val) in board_line.iter().enumerate() {
                board[board_count][column] = *val;
            }
            board_count += 1;
        }

        assert_eq!(38913, play_bingo(&mut bingo_boards, &draws));
    }

    #[test]
    fn test_example_last_winner_bingo() {
        let draws = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = [
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ],
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ],
        ];
        let mut bingo_boards: Vec<Board<5, 5>> = Vec::new();

        for board in boards {
            bingo_boards.push(Board::build_board_from_array(&board));
        }
        assert_eq!(1924, play_bingo_win_last(&mut bingo_boards, &draws));
    }

    #[test]
    fn test_bingo_last_winner() {
        let mut bingo_boards: Vec<Board<5, 5>> = Vec::new();
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let mut board = [[0; 5]; 5];
        let mut first = true;
        let mut draws: Vec<u32> = Vec::new();
        let mut board_count = 0;
        for line in contents.lines() {
            if first {
                draws = line.split(',').map(|c| c.parse::<u32>().unwrap()).collect();
                first = false;
                continue;
            }

            if line.is_empty() {
                bingo_boards.push(Board::build_board_from_array(&board));
                board_count = 0;
                continue;
            }

            let board_line: Vec<u32> = line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect();
            for (column, val) in board_line.iter().enumerate() {
                board[board_count][column] = *val;
            }
            board_count += 1;
        }

        assert_eq!(38913, play_bingo_win_last(&mut bingo_boards, &draws));
    }
}

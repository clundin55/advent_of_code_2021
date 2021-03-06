pub const DAY_THREE_INPUT: &str = "puzzle_input.txt";

pub fn calculate_power_consumption<const N: usize, const M: usize>(
    diagnostics: &[[u32; M]; N],
) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    let n = diagnostics.len();
    let m = diagnostics[0].len();

    for y in 0..m {
        let mut one_count = 1;
        let mut zero_count = 1;
        for x in 0..n {
            let val = diagnostics[x][y];
            if val == 1 {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if one_count > zero_count {
            gamma = gamma | 1;
        } else {
            epsilon = epsilon | 1;
        }
    }

    gamma * epsilon
}

pub fn calculate_life_system(diagnostics: &[i32], width: usize) -> i32 {
    let mut oxygen_list = diagnostics.to_owned();
    for i in (0..width).rev() {
        let mut zero_count = 0;
        let mut one_count = 0;
        if oxygen_list.len() == 1 {
            break;
        }
        for x in &oxygen_list {
            let y = x >> i;
            if y & 1 == 1 {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        let counter = 1 << i;

        if one_count > zero_count {
            oxygen_list = oxygen_list
                .into_iter()
                .filter(|x| *x & counter == counter)
                .collect();
        } else if one_count < zero_count {
            oxygen_list = oxygen_list
                .into_iter()
                .filter(|x| *x & counter != counter)
                .collect();
        } else {
            oxygen_list = oxygen_list
                .into_iter()
                .filter(|x| *x & counter == counter)
                .collect();
        }
    }

    let mut co2_list = diagnostics.to_owned();
    for i in (0..width).rev() {
        let mut zero_count = 0;
        let mut one_count = 0;
        if co2_list.len() == 1 {
            break;
        }

        for x in &co2_list {
            let y = x >> i;
            if y & 1 == 1 {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        let counter = 1 << i;

        if one_count > zero_count {
            co2_list = co2_list
                .into_iter()
                .filter(|x| *x & counter != counter)
                .collect();
        } else if one_count < zero_count {
            co2_list = co2_list
                .into_iter()
                .filter(|x| *x & counter == counter)
                .collect();
        } else {
            co2_list = co2_list
                .into_iter()
                .filter(|x| *x & counter != counter)
                .collect();
        }
    }

    let oxygen = *oxygen_list.last().unwrap();
    let co2 = *co2_list.last().unwrap();
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_example_power_consumption() {
        let diagnostics = [
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 1, 1, 0],
            [1, 0, 1, 1, 1],
            [1, 0, 1, 0, 1],
            [0, 1, 1, 1, 1],
            [0, 0, 1, 1, 1],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];
        assert_eq!(198, calculate_power_consumption(&diagnostics));
    }

    #[test]
    fn test_power_consumption() {
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        let mut diagnostics = [[0; 12]; 1000];

        file.read_to_string(&mut contents).unwrap();

        let temp_diagnostics: Vec<Vec<u32>> = contents
            .lines()
            .map(|v| v.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        for (i, n) in temp_diagnostics.into_iter().enumerate() {
            for (j, m) in n.into_iter().enumerate() {
                diagnostics[i][j] = m;
            }
        }
        assert_eq!(1131506, calculate_power_consumption(&diagnostics));
    }

    #[test]
    fn test_example_lifesupport() {
        let diagnostics = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(230, calculate_life_system(&diagnostics, 5));
    }

    #[test]
    fn test_lifesupport() {
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let diagnostics: Vec<i32> = contents
            .lines()
            .map(|s| i32::from_str_radix(s, 2).unwrap())
            .collect();

        assert_eq!(7863147, calculate_life_system(&diagnostics, 12));
    }
}

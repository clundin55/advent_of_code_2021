const DAY_THREE_INPUT: &str = "puzzle_input.txt";

fn calculate_power_consumption(diagnostics: &[i32], width: usize) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut one_count = vec![0; width];
    let mut zero_count = vec![0; width];
    for x in diagnostics {
        let mut z = 1;
        for i in 0..width {
            if z & x == z {
                one_count[i] += 1;
            } else {
                zero_count[i] += 1;
            }

            z = z << 1;
        }
    }

    let mut z = 1;
    for i in (0..one_count.len()).rev() {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if one_count[i] > zero_count[i] {
            gamma = gamma | z;
        } else {
            epsilon = epsilon | z;
        }
        println!("gamma: {:#b}", gamma);
        println!("epsilon: {:#b}", epsilon);
    }

    gamma * epsilon
}

fn calculate_life_system(diagnostics: &[i32], width: usize) -> i32 {
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
        println!("i: {:#b}", i);
        println!("counter: {:#b}", counter);

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
            let mut y = x >> i;
            println!("y: {:#b}", x);
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
        let diagnostics = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(198, calculate_power_consumption(&diagnostics, 5));
    }

    #[test]
    fn test_power_consumption() {
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let diagnostics: Vec<i32> = contents
            .lines()
            .map(|s| i32::from_str_radix(s, 2).unwrap())
            .collect();
        assert_eq!(1131506, calculate_power_consumption(&diagnostics, 12));
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

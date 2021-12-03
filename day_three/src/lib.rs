const DAY_THREE_INPUT: &str = "puzzle_input.txt";

fn calculate_power_consumption(diagnostics: &[i32], width: usize) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut one_count = vec![0; width];
    let mut zero_count = vec![0; width];
    dbg!(diagnostics);
    for x in diagnostics {
        let mut z = 1;
        for i in 0..width {
            //dbg!(z);
            //dbg!(x);
            if z & x == z {
                one_count[i] += 1;
            } else {
                zero_count[i] += 1;
            }

            z = z << 1;
        }
    }

    dbg!(&one_count);
    dbg!(&zero_count);

    let mut z = 1;
    for i in (0..one_count.len()).rev() {
        dbg!(i);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_example_power_consumption() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(198, calculate_power_consumption(&input, 5));
    }

    #[test]
    fn test_course_charting_with_aim() {
        let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let course: Vec<i32> = contents.lines()
            .map(|s| i32::from_str_radix(s,2).unwrap())
            .collect();
        assert_eq!(1971232560, calculate_power_consumption(&course, 12));
    }
}

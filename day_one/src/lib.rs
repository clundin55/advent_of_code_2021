const DAY_ONE_INPUT: &str = "puzzle_input.txt";

fn depth_increases(measurements: &[i32]) -> i32 {
    let mut depth_increases = -1;
    let mut last_measurement = 0;
    for measurement in measurements {
        if *measurement > last_measurement {
            depth_increases += 1;
        }
        last_measurement = *measurement;
    }
    depth_increases
}

fn sliding_window_depth_increases(measurements: &[i32], window: usize) -> i32 {
    let mut depth_increases = 0;
    for idx in 0..measurements.len() - window {
        if measurements[idx] < measurements[idx + window] {
            depth_increases += 1;
        }
    }

    depth_increases
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_depth_increases() {
        let mut file = File::open(DAY_ONE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let depths: Vec<i32> = contents
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let answer = depth_increases(&depths);
        assert_eq!(1184, answer);
    }

    #[test]
    fn test_sliding_window_depth_increases() {
        let mut file = File::open(DAY_ONE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let depths: Vec<i32> = contents
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(1158, sliding_window_depth_increases(&depths, 3));
    }

    #[test]
    fn test_example_sliding_window_depth_increases() {
        let depths: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, sliding_window_depth_increases(&depths, 3));
    }
}

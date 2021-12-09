use std::cmp::{max, min};

fn crab_horizontal_cost(initial_crab_positions: &[u32]) -> u32 {
    let mut crab_positions = initial_crab_positions.to_vec();
    crab_positions.sort();
    let median = crab_positions[crab_positions.len() / 2] as i32;
    crab_positions.iter().map(|x| (*x as i32 - median).abs()).sum::<i32>() as u32
}

fn crab_horizontal_cost_fuel_drag(initial_crab_positions: &[u32]) -> u32 {
    let mean = initial_crab_positions.iter().sum::<u32>() as f32 / initial_crab_positions.len() as f32;
    let ceil = initial_crab_positions.iter().map(|x| (0..=(*x as i32 - mean.ceil() as i32).abs()).sum::<i32>()).sum::<i32>() as u32;
    let floor = initial_crab_positions.iter().map(|x| (0..=(*x as i32 - mean.floor() as i32).abs()).sum::<i32>()).sum::<i32>() as u32;
    min(ceil,floor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const DAY_SEVEN_INPUT: &str = "puzzle_input.txt";

    #[test]
    fn example_crab_positions() {
        let crab_positions = [16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, crab_horizontal_cost(&crab_positions));
    }

    #[test]
    fn crab_positions() {
        let mut file = File::open(DAY_SEVEN_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let crab_positions: Vec<u32> = contents
            .split(",")
            .map(|c| c.trim().parse().unwrap())
            .collect();
        assert_eq!(355521, crab_horizontal_cost(&crab_positions));
    }

    #[test]
    fn example_crab_positions_with_drag() {
        let crab_positions = [16,1,2,0,4,2,7,1,2,14];
        assert_eq!(168, crab_horizontal_cost_fuel_drag(&crab_positions));
    }

    #[test]
    fn crab_positions_with_drag() {
        let mut file = File::open(DAY_SEVEN_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let crab_positions: Vec<u32> = contents
            .split(",")
            .map(|c| c.trim().parse().unwrap())
            .collect();
        assert_eq!(100148777, crab_horizontal_cost_fuel_drag(&crab_positions));
    }
}

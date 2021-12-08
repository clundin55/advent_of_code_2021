const DAY_SIX_INPUT: &str = "puzzle_input.txt";

fn step_simulation(fishes: &mut Vec<u64>) {
    let new_fish = fishes[0];
    fishes.rotate_left(1);
    fishes[6] += new_fish;
    fishes[8] = new_fish;
}

fn simulate_lanternfish_growth(starting_fish: &[u64], days: u64) -> u64 {
    let mut fishes: Vec<u64> = vec![0; 9];
    for fish in starting_fish {
        fishes[*fish as usize] += 1;
    }

    for day in 0..days {
        step_simulation(&mut fishes);
    }

    println!("{}", fishes.len());
    fishes.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn example_fish_growth_simulation() {
        let initial_fish = vec![3, 4, 3, 1, 2];
        assert_eq!(26, simulate_lanternfish_growth(&initial_fish, 18));
        assert_eq!(5934, simulate_lanternfish_growth(&initial_fish, 80));
        assert_eq!(26984457539, simulate_lanternfish_growth(&initial_fish, 256));
    }

    #[test]
    fn fish_growth_simulation() {
        let mut file = File::open(DAY_SIX_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let initial_fish: Vec<u64> = contents
            .split(",")
            .map(|c| c.trim().parse().unwrap())
            .collect();
        assert_eq!(379114, simulate_lanternfish_growth(&initial_fish, 80));
        assert_eq!(
            1702631502303,
            simulate_lanternfish_growth(&initial_fish, 256)
        );
    }
}

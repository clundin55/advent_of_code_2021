enum Digit {
    One,
    Four,
    Seven,
    Eight,
}

impl Digit {
    fn digit_size(&self) -> u32 {
        match self {
            Self::One => 2,
            Self::Four => 4,
            Self::Seven => 3,
            Self::Eight => 7,
        }
    }

    fn size_to_digit(size: &u32) -> Option<Self> {
        match size {
            2 => Some(Self::One),
            4 => Some(Self::Four),
            3 => Some(Self::Seven),
            7 => Some(Self::Eight),
            _ => None,
        }
    }
}

fn find_number_seqments(input: &str) -> Option<u32> {
    let mut unique_num_seqments = 0;
    for slice in input.lines() {
        let digit_sizes = slice
            .split_once("|")?
            .1
            .split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).count() as u32)
            .map(|size| Digit::size_to_digit(&size))
            .filter(|x| x.is_some())
            .count() as u32;
        unique_num_seqments += digit_sizes
    }

    Some(unique_num_seqments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    const PUZZLE_EXAMPLE: &str = "example_input.txt";
    const PUZZLE_INPUT: &str = "puzzle_input.txt";

    #[test]
    fn example_seven_segment() {
        let mut file = File::open(PUZZLE_EXAMPLE.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        assert_eq!(Some(26), find_number_seqments(&contents));
    }

    #[test]
    fn seven_segment() {
        let mut file = File::open(PUZZLE_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        assert_eq!(Some(342), find_number_seqments(&contents));
    }
}

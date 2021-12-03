enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn from_tuple((direction, distance): (&str, i32)) -> Self {
        match direction.to_string().to_lowercase().as_str() {
            "forward" => Direction::Forward(distance),
            "down" => Direction::Down(distance),
            _ => Direction::Up(distance),
        }
    }
}

const DAY_TWO_INPUT: &str = "puzzle_input.txt";

fn plan_course(course: &[Direction]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for direction in course {
        match direction {
            Direction::Forward(x) => horizontal_pos += x,
            Direction::Up(x) => depth -= x,
            Direction::Down(x) => depth += x,
        }
    }

    depth * horizontal_pos
}

fn plan_course_with_aim(course: &[Direction]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in course {
        match direction {
            Direction::Forward(x) => {
                horizontal_pos += x;
                depth += aim * x;
            },
            Direction::Up(x) => aim -= x,
            Direction::Down(x) => aim += x,
        }
    }

    depth * horizontal_pos
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_course_charting() {
        let mut file = File::open(DAY_TWO_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let course: Vec<Direction> = contents
            .split("\n")
            .map(|s| s.split_once(" "))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .map(|s| (s.0, s.1.parse().unwrap()))
            .map(|(direction, distance)| Direction::from_tuple((direction, distance)))
            .collect();
        assert_eq!(1882980, plan_course(&course));
    }

    #[test]
    fn test_example_course_charting() {
        let course: Vec<Direction> = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ]
        .iter()
        .map(|e| Direction::from_tuple(*e))
        .collect();
        assert_eq!(150, plan_course(&course));
    }

    #[test]
    fn test_course_charting_with_aim() {
        let mut file = File::open(DAY_TWO_INPUT.to_string()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let course: Vec<Direction> = contents
            .split("\n")
            .map(|s| s.split_once(" "))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .map(|s| (s.0, s.1.parse().unwrap()))
            .map(|(direction, distance)| Direction::from_tuple((direction, distance)))
            .collect();
        assert_eq!(1971232560, plan_course_with_aim(&course));
    }

    #[test]
    fn test_example_course_charting_with_aim() {
        let course: Vec<Direction> = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ]
        .iter()
        .map(|e| Direction::from_tuple(*e))
        .collect();
        assert_eq!(900, plan_course_with_aim(&course));
    }
}

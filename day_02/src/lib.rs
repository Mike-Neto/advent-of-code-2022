use std::{fs, io::Error};

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char(char: &str) -> Option<Choice> {
        match char {
            "A" | "X" => Some(Choice::Rock),
            "B" | "Y" => Some(Choice::Paper),
            "C" | "Z" => Some(Choice::Scissors),
            _ => None,
        }
    }

    fn play_against(&self, me: &Self) -> u64 {
        match me {
            Choice::Rock => match self {
                Choice::Rock => 4,
                Choice::Paper => 1,
                Choice::Scissors => 7,
            },
            Choice::Paper => match self {
                Choice::Paper => 5,
                Choice::Scissors => 2,
                Choice::Rock => 8,
            },
            Choice::Scissors => match self {
                Choice::Scissors => 6,
                Choice::Paper => 9,
                Choice::Rock => 3,
            },
        }
    }

    fn play_against_2(&self, outcome: &Self) -> u64 {
        // Choice::Rock Lose!
        // Choice::Paper Draw!
        // Choice::Scissors Win!
        match outcome {
            Choice::Rock => match self {
                Choice::Rock => 3,
                Choice::Paper => 1,
                Choice::Scissors => 2,
            },
            Choice::Paper => match self {
                Choice::Paper => 5,
                Choice::Scissors => 6,
                Choice::Rock => 4,
            },
            Choice::Scissors => match self {
                Choice::Scissors => 7,
                Choice::Paper => 9,
                Choice::Rock => 8,
            },
        }
    }
}

pub fn day_two_part_one(path: &str) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .map(|round| {
            round
                .split_whitespace()
                .take(2)
                .filter_map(|char| Choice::from_char(char))
                .collect()
        })
        .map(|round: Vec<Choice>| round[0].play_against(&round[1]))
        .sum();

    Ok(score)
}

pub fn day_two_part_two(path: &str) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .map(|round| {
            round
                .split_whitespace()
                .take(2)
                .filter_map(|char| Choice::from_char(char))
                .collect()
        })
        .map(|round: Vec<Choice>| round[0].play_against_2(&round[1]))
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use crate::{day_two_part_one, day_two_part_two};

    #[test]
    fn day_two_part_one_example() {
        let result = day_two_part_one("example.txt").unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn day_two_part_one_data() {
        let result = day_two_part_one("data.txt").unwrap();
        assert!(result < 12149);
        assert_eq!(result, 11603);
    }

    #[test]
    fn day_two_part_two_example() {
        let result = day_two_part_two("example.txt").unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn day_two_part_two_data() {
        let result = day_two_part_two("data.txt").unwrap();
        assert!(result > 11915);
        assert_eq!(result, 12);
    }
}

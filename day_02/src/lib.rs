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
}

pub fn day_one_part_one(path: &str) -> Result<u64, Error> {
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

#[cfg(test)]
mod tests {
    use crate::day_one_part_one;

    #[test]
    fn day_one_part_one_example() {
        let result = day_one_part_one("example.txt").unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn day_one_part_one_data() {
        let result = day_one_part_one("data.txt").unwrap();
        assert!(result < 12149);
        assert_eq!(result, 11603);
    }
}

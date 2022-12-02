use std::{fs, io::Error};

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(char: &str) -> Option<Outcome> {
        match char {
            "X" => Some(Outcome::Lose),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

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

    fn value(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

fn outcome_from_choices(opponent: &Choice, player: &Choice) -> Outcome {
    match player {
        Choice::Rock => match opponent {
            Choice::Rock => Outcome::Draw,
            Choice::Paper => Outcome::Lose,
            Choice::Scissors => Outcome::Win,
        },
        Choice::Paper => match opponent {
            Choice::Paper => Outcome::Draw,
            Choice::Scissors => Outcome::Lose,
            Choice::Rock => Outcome::Win,
        },
        Choice::Scissors => match opponent {
            Choice::Scissors => Outcome::Draw,
            Choice::Paper => Outcome::Win,
            Choice::Rock => Outcome::Lose,
        },
    }
}

fn score_from_round(opponent: &Choice, player: &Choice) -> u64 {
    let outcome = outcome_from_choices(opponent, player);

    outcome.value() + player.value()
}

fn choice_from_opponent_and_outcome(opponent: &Choice, outcome: &Outcome) -> Choice {
    match opponent {
        Choice::Rock => match outcome {
            Outcome::Draw => Choice::Rock,
            Outcome::Lose => Choice::Scissors,
            Outcome::Win => Choice::Paper,
        },
        Choice::Paper => match outcome {
            Outcome::Draw => Choice::Paper,
            Outcome::Lose => Choice::Rock,
            Outcome::Win => Choice::Scissors,
        },
        Choice::Scissors => match outcome {
            Outcome::Draw => Choice::Scissors,
            Outcome::Lose => Choice::Paper,
            Outcome::Win => Choice::Rock,
        },
    }
}

fn score_from_round_2(opponent: &Choice, outcome: &Outcome) -> u64 {
    let player = choice_from_opponent_and_outcome(opponent, outcome);

    outcome.value() + player.value()
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
        .map(|round: Vec<Choice>| score_from_round(&round[0], &round[1]))
        .sum();

    Ok(score)
}

pub fn day_two_part_two(path: &str) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .filter_map(|round| {
            let round: Vec<&str> = round.split_whitespace().take(2).collect();
            let choice = Choice::from_char(round[0]);
            let outcome = Outcome::from_char(round[1]);
            match (choice, outcome) {
                (Some(choice), Some(outcome)) => Some((choice, outcome)),
                _ => None,
            }
        })
        .map(|(choice, outcome)| score_from_round_2(&choice, &outcome))
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
        assert_eq!(result, 11603);
        assert!(result < 12149);
    }

    #[test]
    fn day_two_part_two_example() {
        let result = day_two_part_two("example.txt").unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn day_two_part_two_data() {
        let result = day_two_part_two("data.txt").unwrap();
        assert_eq!(result, 12725);
        assert!(result > 11915);
    }
}

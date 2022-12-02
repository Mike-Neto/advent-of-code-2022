use std::{fs, io::Error};

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

pub enum Match {
    Choice,
    Outcome,
}

impl Choice {
    fn from_char(char: &str) -> Option<Self> {
        match char {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None,
        }
    }

    const fn value(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Outcome {
    fn from_char(char: &str) -> Option<Self> {
        match char {
            "X" => Some(Self::Lose),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None,
        }
    }

    const fn value(&self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

const fn outcome_from_choices(opponent: &Choice, player: &Choice) -> Outcome {
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

const fn score_from_choices(opponent: &Choice, player: &Choice) -> u64 {
    let outcome = outcome_from_choices(opponent, player);

    outcome.value() + player.value()
}

const fn choice_from_opponent_and_outcome(opponent: &Choice, outcome: &Outcome) -> Choice {
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

const fn score_from_choice_and_outcome(opponent: &Choice, outcome: &Outcome) -> u64 {
    let player = choice_from_opponent_and_outcome(opponent, outcome);

    outcome.value() + player.value()
}

/// Runs each round based on the `match_type` interpretation of the second column
/// and sums up the score of the whole match.
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn score_from_match_type(path: &str, match_type: &Match) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .filter_map(|round| {
            let round: Vec<&str> = round.split_whitespace().take(2).collect();
            let choice = Choice::from_char(round[0]);
            let match_type_value = match match_type {
                Match::Choice => (Choice::from_char(round[1]), None),
                Match::Outcome => (None, Outcome::from_char(round[1])),
            };
            match (choice, match_type_value) {
                (Some(choice), (Some(player_choice), None)) => {
                    Some(score_from_choices(&choice, &player_choice))
                }
                (Some(choice), (None, Some(round_outcome))) => {
                    Some(score_from_choice_and_outcome(&choice, &round_outcome))
                }
                _ => None,
            }
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use crate::{score_from_match_type, Match};

    #[test]
    fn day_two_part_one_example() {
        let result = score_from_match_type("example.txt", &Match::Choice).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn day_two_part_one_data() {
        let result = score_from_match_type("data.txt", &Match::Choice).unwrap();
        assert_eq!(result, 11603);
        assert!(result < 12149);
    }

    #[test]
    fn day_two_part_two_example() {
        let result = score_from_match_type("example.txt", &Match::Outcome).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn day_two_part_two_data() {
        let result = score_from_match_type("data.txt", &Match::Outcome).unwrap();
        assert_eq!(result, 12725);
        assert!(result > 11915);
    }
}

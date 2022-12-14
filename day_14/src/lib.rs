use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult as NomResult, Parser,
};
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    x: i64,
    y: i64,
}

pub enum Move {
    Down,
    DownLeft,
    DownRight,
    OutOfBounds,
    Settled,
    BlockedSource,
}

impl Position {
    const DOWN: Position = Self { x: 0, y: 1 };
    const DOWN_LEFT: Position = Self { x: -1, y: 1 };
    const DOWN_RIGHT: Position = Self { x: 1, y: 1 };
    const SOURCE: Position = Self { x: 500, y: 0 };

    fn simulate_move(
        &self,
        rested_blocks: &Vec<Position>,
        x_bound: Option<i64>,
        y_bound: i64,
    ) -> (Option<Position>, Move) {
        let moves = vec![Position::DOWN, Position::DOWN_LEFT, Position::DOWN_RIGHT];
        for (index, m) in moves.iter().enumerate() {
            let target = Position {
                x: self.x + m.x,
                y: self.y + m.y,
            };
            let is_target_free = !rested_blocks.iter().cloned().any(|pos| pos == target);
            let is_floor = x_bound.is_none() && target.y == y_bound;
            if is_target_free && !is_floor {
                let is_in_bounds = if let Some(x_bound) = x_bound {
                    target.x >= 0 && target.x <= x_bound && target.y >= 0 && target.y <= y_bound
                } else {
                    true
                };
                if !is_in_bounds {
                    return (None, Move::OutOfBounds);
                } else {
                    let target_move = match index {
                        0 => Move::Down,
                        1 => Move::DownLeft,
                        _ => Move::DownRight,
                    };
                    return (Some(target), target_move);
                }
            }
        }

        if *self == Position::SOURCE {
            return (None, Move::BlockedSource);
        }

        return (None, Move::Settled);
    }

    fn cartesian_product(&self, other: &Self) -> Vec<Position> {
        let mut result = vec![];

        for x in 0..=self.x.abs_diff(other.x) {
            for y in 0..=self.y.abs_diff(other.y) {
                let x = self.x.min(other.x) + x as i64;
                let y = self.y.min(other.y) + y as i64;
                result.push(Position { x, y });
            }
        }

        result
    }
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Vec<Position>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(complete::i64, tag(","), complete::i64).map(|(x, y)| Position { x, y }),
        ),
    )(input)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_fourteen_part_one(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, rock_vectors) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;
    let mut rested_positions: Vec<Position> = rock_vectors
        .iter()
        .map(|vectors| {
            let v: Vec<Vec<Position>> = vectors
                .windows(2)
                .map(|window| window[0].cartesian_product(&window[1]))
                .collect();
            v
        })
        .flatten()
        .flatten()
        .collect();

    let x_bound = rested_positions
        .iter()
        .map(|p| p.x)
        .max()
        .unwrap_or_default();
    let y_bound = rested_positions
        .iter()
        .map(|p| p.y)
        .max()
        .unwrap_or_default();

    let mut sand_units = 0;
    let mut out_of_bounds = false;
    loop {
        let mut sand_unit = Position::SOURCE.clone();
        sand_units += 1;

        loop {
            let (new_position, move_outcome) =
                sand_unit.simulate_move(&rested_positions, Some(x_bound), y_bound);
            match move_outcome {
                Move::OutOfBounds => {
                    out_of_bounds = true;
                    sand_units -= 1;
                    break;
                }
                Move::Settled => {
                    rested_positions.push(sand_unit);
                    break;
                }
                _ => {
                    sand_unit = new_position.expect("moved to new position");
                }
            }
        }

        if out_of_bounds {
            break;
        }
    }

    Ok(sand_units)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_fourteen_part_two(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, rock_vectors) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;
    let mut rested_positions: Vec<Position> = rock_vectors
        .iter()
        .map(|vectors| {
            let v: Vec<Vec<Position>> = vectors
                .windows(2)
                .map(|window| window[0].cartesian_product(&window[1]))
                .collect();
            v
        })
        .flatten()
        .flatten()
        .collect();

    let y_bound = rested_positions
        .iter()
        .map(|p| p.y)
        .max()
        .unwrap_or_default()
        + 2;

    let mut sand_units = 0;
    let mut blocked_source = false;
    loop {
        let mut sand_unit = Position::SOURCE.clone();
        sand_units += 1;

        loop {
            let (new_position, move_outcome) =
                sand_unit.simulate_move(&rested_positions, None, y_bound);
            match move_outcome {
                Move::BlockedSource => {
                    blocked_source = true;
                    break;
                }
                Move::Settled => {
                    rested_positions.push(sand_unit);
                    break;
                }
                _ => {
                    sand_unit = new_position.expect("moved to new position");
                }
            }
        }

        if blocked_source {
            break;
        }
    }

    Ok(sand_units)
}

#[cfg(test)]
mod tests {
    use crate::{day_fourteen_part_one, day_fourteen_part_two};

    #[test]
    fn day_fourteen_part_one_example() {
        let result = day_fourteen_part_one("example.txt").unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn day_fourteen_part_one_data() {
        let result = day_fourteen_part_one("data.txt").unwrap();
        assert_eq!(result, 755);
    }

    #[test]
    fn day_fourteen_part_two_example() {
        let result = day_fourteen_part_two("example.txt").unwrap();
        assert_eq!(result, 93);
    }

    #[test]
    #[ignore = "it takes a long time to run at the moment"]
    fn day_fourteen_part_two_data() {
        let result = day_fourteen_part_two("data.txt").unwrap();
        assert_eq!(result, 29805);
    }
}

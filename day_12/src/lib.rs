use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult as NomResult,
};
use pathfinding::prelude::astar;
use std::fs::read_to_string;
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Vec<char>>> {
    let (input, monkeys) =
        separated_list1(newline, many1(one_of("qwertyuiopasdfghjklzxcvbnmSE")))(input)?;

    Ok((input, monkeys))
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    const fn distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn successors(&self, height_map: &Vec<Vec<i16>>) -> Vec<(Self, usize)> {
        let &Self(x, y) = self;
        let mut cross_positions: Vec<(Self, usize)> = vec![];
        let current_height = height_map[y][x];

        if x >= 1 {
            let target_x = x - 1;
            let target_height = height_map[y][target_x];
            let diff = target_height - current_height;
            if diff <= 1 {
                cross_positions.push((Self(target_x, y), 1));
            }
        }
        if x < height_map[y].len() - 1 {
            let target_x = x + 1;
            let target_height = height_map[y][target_x];
            let diff = target_height - current_height;
            if diff <= 1 {
                cross_positions.push((Self(target_x, y), 1));
            }
        }
        if y >= 1 {
            let target_y = y - 1;
            let target_height = height_map[target_y][x];
            let diff = target_height - current_height;
            if diff <= 1 {
                cross_positions.push((Self(x, target_y), 1));
            }
        }
        if y < height_map.len() - 1 {
            let target_y = y + 1;
            let target_height = height_map[target_y][x];
            let diff = target_height - current_height;
            if diff <= 1 {
                cross_positions.push((Self(x, target_y), 1));
            }
        }

        cross_positions
    }
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_twelve_part_one(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, height_map) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let starting_position = height_map.iter().flatten().position(|&c| c == 'S');
    let end_position = height_map.iter().flatten().position(|&c| c == 'E');

    if let (Some(starting_position), Some(end_position)) = (starting_position, end_position) {
        let starting_x = starting_position % (height_map[0].len());
        let starting_y = starting_position / (height_map[0].len());
        let end_x = end_position % (height_map[0].len());
        let end_y = end_position / (height_map[0].len());
        let start = Pos(starting_x, starting_y);
        let goal = Pos(end_x, end_y);

        let numeric_height_map: Vec<Vec<i16>> = height_map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        'S' => 'a' as i16,
                        'E' => 'z' as i16,
                        _ => *c as i16,
                    })
                    .collect()
            })
            .collect();

        let result = astar(
            &start,
            |p| p.successors(&numeric_height_map),
            |p| p.distance(&goal),
            |p| *p == goal,
        );

        if let Some(result) = result {
            return Ok(result.0.len() - 1);
        }
    }

    Ok(0)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_twelve_part_two(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, height_map) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let end_position = height_map.iter().flatten().position(|&c| c == 'E');

    let numeric_height_map: Vec<Vec<i16>> = height_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    'S' => 'a' as i16,
                    'E' => 'z' as i16,
                    _ => *c as i16,
                })
                .collect()
        })
        .collect();

    let mut shortest_len = usize::MAX;
    for (index, _) in height_map
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, &c)| c == 'a' || c == 'S')
    {
        if let (starting_position, Some(end_position)) = (index, end_position) {
            let starting_x = starting_position % (height_map[0].len());
            let starting_y = starting_position / (height_map[0].len());
            let end_x = end_position % (height_map[0].len());
            let end_y = end_position / (height_map[0].len());
            let start = Pos(starting_x, starting_y);
            let goal = Pos(end_x, end_y);

            let result = astar(
                &start,
                |p| p.successors(&numeric_height_map),
                |p| p.distance(&goal),
                |p| *p == goal,
            );

            if let Some(result) = result {
                shortest_len = shortest_len.min(result.0.len() - 1);
            }
        }
    }

    Ok(shortest_len)
}

#[cfg(test)]
mod tests {
    use crate::{day_twelve_part_one, day_twelve_part_two};

    #[test]
    fn day_twelve_part_one_example() {
        let result = day_twelve_part_one("example.txt").unwrap();
        assert_eq!(result, 31);
    }

    #[test]
    fn day_twelve_part_one_data() {
        let result = day_twelve_part_one("data.txt").unwrap();
        assert_eq!(result, 408);
    }

    #[test]
    fn day_twelve_part_two_example() {
        let result = day_twelve_part_two("example.txt").unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    fn day_twelve_part_two_data() {
        let result = day_twelve_part_two("data.txt").unwrap();
        assert_eq!(result, 399);
    }
}

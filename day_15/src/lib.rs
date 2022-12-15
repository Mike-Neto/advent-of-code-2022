use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult as NomResult,
};
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Pair {
    sensor: Position,
    beacon: Position,
}

// at x=2, y=18
fn parse_position(input: &str) -> NomResult<&str, Position> {
    let (input, _) = tag("at ")(input)?;
    let (input, (x, y)) = separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)?;

    Ok((input, Position { x, y }))
}

fn parse_line(input: &str) -> NomResult<&str, Pair> {
    let (input, _) = tag("Sensor ")(input)?;
    let (input, sensor) = parse_position(input)?;
    let (input, _) = tag(": closest beacon is ")(input)?;
    let (input, beacon) = parse_position(input)?;

    Ok((input, Pair { sensor, beacon }))
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Pair>> {
    separated_list1(newline, parse_line)(input)
}

fn print_grid(grid: &HashMap<Position, char>) {
    let lower_x = grid.keys().map(|p| p.x).min().unwrap_or_default();
    let upper_x = grid.keys().map(|p| p.x).max().unwrap_or_default();
    let lower_y = grid.keys().map(|p| p.y).min().unwrap_or_default();
    let upper_y = grid.keys().map(|p| p.y).max().unwrap_or_default();
    for y in lower_y..=upper_y {
        print!("{y:0>2} ");
        for x in lower_x..=upper_x {
            let symbol = grid.get(&Position { x, y }).unwrap_or(&'.');
            print!("{symbol}");
        }
        print!("\n");
    }
    print!("\n");
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_fifteen_part_one(path: &str, target_y: i64) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, sensor_beacon_pairs) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut grid: HashMap<Position, char> = HashMap::new();

    for Pair { sensor, beacon } in sensor_beacon_pairs {
        let x_distance = sensor.x.abs_diff(beacon.x);
        let y_distance = sensor.y.abs_diff(beacon.y);
        let absolute_distance = (x_distance + y_distance) as i64;
        let y_distance_to_target = sensor.y.abs_diff(target_y);

        let y_range = (sensor.y - absolute_distance)..=(sensor.y + absolute_distance);
        if y_range.contains(&target_y) {
            let x_range = (sensor.x
                - absolute_distance.abs_diff(y_distance_to_target as i64) as i64)
                ..=(sensor.x + absolute_distance.abs_diff(y_distance_to_target as i64) as i64);
            for x in x_range {
                grid.insert(Position { x, y: target_y }, '#');
            }
        }

        grid.insert(sensor, 'S');
        grid.insert(beacon, 'B');
    }

    let mut count = 0;
    let lower_x = grid.keys().map(|p| p.x).min().unwrap_or_default();
    let upper_x = grid.keys().map(|p| p.x).max().unwrap_or_default();
    for x in lower_x..=upper_x {
        if let Some(c) = grid.get(&Position { x, y: target_y }) {
            if *c == '#' {
                count += 1;
            }
        }
    }

    Ok(count)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_fifteen_part_two(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, _) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{day_fifteen_part_one, day_fifteen_part_two};

    #[test]
    #[ignore = "it takes too long"]
    fn day_fifteen_part_one_example() {
        let result = day_fifteen_part_one("example.txt", 10).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    #[ignore = "it takes too long"]
    fn day_fifteen_part_one_data() {
        let result = day_fifteen_part_one("data.txt", 2_000_000).unwrap();
        assert_eq!(result, 4_985_193);
    }

    #[test]
    fn day_fifteen_part_two_example() {
        let result = day_fifteen_part_two("example.txt").unwrap();
        assert_eq!(result, 56_000_011);
    }

    #[test]
    #[ignore]
    fn day_fifteen_part_two_data() {
        let result = day_fifteen_part_two("data.txt").unwrap();
        assert_eq!(result, 56000011);
    }
}

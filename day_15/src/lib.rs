use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult as NomResult,
};
use rayon::prelude::*;
use spiral::ManhattanIterator;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    fs::read_to_string,
    ops::{AddAssign, RangeInclusive},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
pub struct Position {
    x: i64,
    y: i64,
}

unsafe impl Sync for Position {}

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
    println!("{lower_x}->{upper_x}");
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
pub fn day_fifteen_part_two(path: &str, upper_bound: i64) -> Result<i64, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, sensor_beacon_pairs) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let distances: BTreeMap<&Position, i64> = sensor_beacon_pairs
        .iter()
        .map(|Pair { sensor, beacon }| {
            (
                sensor,
                (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs(),
            )
        })
        .collect();

    let mut low_high: BTreeMap<i64, Vec<RangeInclusive<i64>>> = BTreeMap::new();
    for (y, range) in distances.iter().flat_map(|(sensor, max_distance)| {
        ((sensor.y - max_distance)..(sensor.y + max_distance)).map(|y| {
            let distance_to_line = sensor.y - y;

            let max_distance_on_line = *max_distance - distance_to_line.abs();

            (
                y,
                ((sensor.x - max_distance_on_line).max(0))
                    ..=((sensor.x + max_distance_on_line).min(upper_bound)),
            )
        })
    }) {
        if y >= 0 && y <= upper_bound {
            low_high
                .entry(y)
                .and_modify(|lh| lh.push(range.clone()))
                .or_insert(vec![range]);
        }
    }

    let (x, y) = low_high
        .into_iter()
        .find_map(|(key, mut ranges)| {
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            let result: (RangeInclusive<i64>, Option<i64>) =
                ranges.iter().fold((0..=0, None), |mut acc, range| {
                    if acc.1.is_some() {
                        return acc;
                    }
                    if acc.0.end() + 1 >= *range.start() {
                        acc.0 = *acc.0.start()..=(*acc.0.end().max(range.end()));
                    } else {
                        acc.1 = Some(acc.0.end() + 1);
                    }

                    acc
                });
            result.1.map(|x| (x, key))
        })
        .unwrap();

    Ok((x * 4_000_000) + y)
}

#[cfg(test)]
mod tests {
    use crate::{day_fifteen_part_one, day_fifteen_part_two};

    #[test]
    fn day_fifteen_part_one_example() {
        let result = day_fifteen_part_one("example.txt", 10).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    #[ignore = "too slow"]
    fn day_fifteen_part_one_data() {
        let result = day_fifteen_part_one("data.txt", 2_000_000).unwrap();
        assert_eq!(result, 4_985_193);
    }

    #[test]
    fn day_fifteen_part_two_example() {
        let result = day_fifteen_part_two("example.txt", 20).unwrap();
        assert_eq!(result, 56_000_011);
    }

    #[test]
    #[ignore = "too slow"]
    fn day_fifteen_part_two_data() {
        let result = day_fifteen_part_two("data.txt", 4_000_000).unwrap();
        assert_eq!(result, 1);
    }
}

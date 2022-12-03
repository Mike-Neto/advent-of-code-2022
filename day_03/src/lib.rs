use array_tool::vec::Intersect;
use std::{fs, io::Error};

fn char_to_priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        return u64::from(c) - 96;
    }
    u64::from(c) - 38
}

const fn is_even(n: usize) -> bool {
    n % 2 == 0
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_three_part_one(path: &str) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .filter_map(|items| {
            if is_even(items.len())
                && items
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase())
            {
                let half = items.len() / 2;
                let first_compartment: Vec<char> = items.chars().take(half).collect();
                let second_compartment: Vec<char> = items.chars().skip(half).take(half).collect();
                let intersection = first_compartment.intersect(second_compartment);
                let mistake = intersection.first();
                if let Some(mistake) = mistake {
                    return Some(char_to_priority(*mistake));
                }
            }
            None
        })
        .sum();

    Ok(score)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_three_part_two(path: &str) -> Result<u64, Error> {
    let file = fs::read_to_string(path)?;
    let group: Vec<&str> = file.lines().collect();
    let score: u64 = group
        .chunks_exact(3)
        .filter_map(|bags| {
            let bags: Vec<Vec<char>> = bags.iter().map(|s| s.chars().collect()).collect();
            let team = bags[0]
                .intersect(bags[1].clone())
                .intersect(bags[2].clone())
                .first()
                .map(|&c| char_to_priority(c));
            team
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::day_three_part_one;
    use crate::{char_to_priority, day_three_part_two};

    #[test]
    fn day_three_part_one_example() {
        let result = day_three_part_one("example.txt").unwrap();
        assert_eq!(result, 157);
    }

    #[test]
    fn char_to_priority_works() {
        let result = char_to_priority('a');
        assert_eq!(result, 1);
        let result = char_to_priority('z');
        assert_eq!(result, 26);
        let result = char_to_priority('A');
        assert_eq!(result, 27);
        let result = char_to_priority('Z');
        assert_eq!(result, 52);
    }

    #[test]
    fn day_three_part_one_data() {
        let result = day_three_part_one("data.txt").unwrap();
        assert_eq!(result, 8123);
    }

    #[test]
    fn day_three_part_two_example() {
        let result = day_three_part_two("example.txt").unwrap();
        assert_eq!(result, 70);
    }

    #[test]
    fn day_three_part_two_data() {
        let result = day_three_part_two("data.txt").unwrap();
        assert_eq!(result, 2620);
    }
}

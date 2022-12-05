#![allow(clippy::option_if_let_else)]
use std::{collections::BTreeMap, fs::read_to_string};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
    Parse(String),
}

#[derive(Debug)]
struct Move<'a> {
    count: usize,
    from_index: &'a str,
    to_index: &'a str,
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_five_part_one(path: &str) -> Result<String, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (stacks, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| Error::Parse("Failed to split stacks and moves".to_string()))?;

    let mut stack_iter = stacks.split_terminator('\n').rev();
    let stack_names: Vec<&str> = stack_iter.next().unwrap_or("").split_whitespace().collect();
    let mut stacks: BTreeMap<&str, Vec<char>> = BTreeMap::new();

    for line in stack_iter {
        let mut index = 1;
        let chars: Vec<char> = line.chars().collect();
        for n in &stack_names {
            let char = chars[index];
            let queue = stacks.get_mut(n);
            if !char.is_whitespace() {
                if let Some(queue) = queue {
                    queue.push(char);
                } else {
                    let queue = vec![char];
                    stacks.insert(n, queue);
                }
            }
            index += 4;
        }
    }

    let moves: Vec<Move> = moves
        .split_terminator('\n')
        .map(|line| {
            let segments: Vec<&str> = line.split_whitespace().collect();

            let count = segments[1].parse().unwrap_or(0);
            let from_index = segments[3];
            let to_index = segments[5];

            Move {
                count,
                from_index,
                to_index,
            }
        })
        .collect();

    for m in moves {
        for _ in 0..m.count {
            let stack_crate;
            {
                stack_crate = stacks.get_mut(m.from_index).and_then(std::vec::Vec::pop);
            }
            let to = stacks.get_mut(m.to_index);
            if let Some(to) = to {
                if let Some(c) = stack_crate {
                    to.push(c);
                }
            }
        }
    }

    let message = stacks
        .iter()
        .map(|(_, queue)| queue.last().expect("needs at least one elem").to_string())
        .collect::<Vec<String>>();

    Ok(message.join(""))
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_five_part_two(path: &str) -> Result<String, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (stacks, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| Error::Parse("Failed to split stacks and moves".to_string()))?;

    let mut stack_iter = stacks.split_terminator('\n').rev();
    let stack_names: Vec<&str> = stack_iter.next().unwrap_or("").split_whitespace().collect();
    let mut stacks: BTreeMap<&str, Vec<char>> = BTreeMap::new();

    for line in stack_iter {
        let mut index = 1;
        let chars: Vec<char> = line.chars().collect();
        for n in &stack_names {
            let char = chars[index];
            let queue = stacks.get_mut(n);
            if !char.is_whitespace() {
                if let Some(queue) = queue {
                    queue.push(char);
                } else {
                    let queue = vec![char];
                    stacks.insert(n, queue);
                }
            }
            index += 4;
        }
    }

    let moves: Vec<Move> = moves
        .split_terminator('\n')
        .map(|line| {
            let segments: Vec<&str> = line.split_whitespace().collect();

            let count = segments[1].parse().unwrap_or(0);
            let from_index = segments[3];
            let to_index = segments[5];

            Move {
                count,
                from_index,
                to_index,
            }
        })
        .collect();

    for m in moves {
        let stack_crates: Vec<char>;
        {
            // POP count
            stack_crates = stacks
                .get_mut(m.from_index)
                .map(|from| {
                    let final_length = from.len() - m.count;
                    let s: Vec<char> = from.drain(final_length..).collect();
                    s
                })
                .unwrap_or_default();
        }
        let to = stacks.get_mut(m.to_index);
        if let Some(to) = to {
            for sc in &stack_crates {
                to.push(*sc);
            }
        }
    }

    let message = stacks
        .iter()
        .map(|(_, queue)| queue.last().expect("needs at least one elem").to_string())
        .collect::<Vec<String>>();

    Ok(message.join(""))
}

#[cfg(test)]
mod tests {
    use crate::{day_five_part_one, day_five_part_two};

    #[test]
    fn day_five_part_one_example() {
        let result = day_five_part_one("example.txt").unwrap();
        assert_eq!(result, format!("CMZ"));
    }

    #[test]
    fn day_five_part_one_data() {
        let result = day_five_part_one("data.txt").unwrap();
        assert_eq!(result, format!("SHMSDGZVC"));
    }

    #[test]
    fn day_five_part_two_example() {
        let result = day_five_part_two("example.txt").unwrap();
        assert_eq!(result, format!("MCD"));
    }

    #[test]
    fn day_five_part_two_data() {
        let result = day_five_part_two("data.txt").unwrap();
        assert_eq!(result, format!("VRZGHDFBQ"));
    }
}

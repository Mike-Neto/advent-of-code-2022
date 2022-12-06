use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, multispace1, newline},
    multi::separated_list1,
    sequence::delimited,
    IResult as NomResult,
};
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

type Stacks<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn parse_crate(input: &str) -> NomResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn parse_move(input: &str) -> NomResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::i64(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_index) = alphanumeric1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_index) = alphanumeric1(input)?;
    Ok((
        input,
        Move {
            count: count.try_into().unwrap_or_default(),
            from_index,
            to_index,
        },
    ))
}

fn parse_input(input: &str) -> NomResult<&str, (Stacks, Vec<Move>)> {
    let (input, crate_rows) =
        separated_list1(newline, separated_list1(tag(" "), parse_crate))(input)?;
    let (input, _) = newline(input)?;
    let (input, stack_names) =
        separated_list1(tag(" "), delimited(tag(" "), alphanumeric1, tag(" ")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    let stacks: BTreeMap<&str, Vec<&str>> = stack_names
        .iter()
        .enumerate()
        .map(|(row_index, &stack_name)| {
            let stack: Vec<&str> = crate_rows
                .iter()
                .rev()
                .filter_map(|row| row[row_index])
                .collect();

            (stack_name, stack)
        })
        .collect();

    Ok((input, (stacks, moves)))
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_five_part_one(path: &str) -> Result<String, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, (mut stacks, moves)) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

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
        .map(|(_, queue)| (*queue.last().expect("needs at least one elem")).to_string())
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
    let (_, (mut stacks, moves)) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    for m in moves {
        let stack_crates: Vec<&str>;
        {
            // POP count
            stack_crates = stacks
                .get_mut(m.from_index)
                .map(|from| {
                    let final_length = from.len() - m.count;
                    let s: Vec<&str> = from.drain(final_length..).collect();
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
        .map(|(_, queue)| (*queue.last().expect("needs at least one elem")).to_string())
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

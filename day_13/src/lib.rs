use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult as NomResult, Parser,
};
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Self>),
    Number(u64),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Self::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Self::Number(*l0)] == r0,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Number(b)) => a.cmp(&vec![Self::Number(*b)]),
            (Self::Number(a), Self::List(b)) => vec![Self::Number(*a)].cmp(b),
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> NomResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")).map(Packet::List),
        nom::character::complete::u64.map(Packet::Number),
    ))(input)
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet, newline, parse_packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_thirteen_part_one(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, pairs) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let right_order_indexes_sum = pairs
        .iter()
        .enumerate()
        .filter_map(|(index, Pair { left, right })| match left.cmp(right) {
            std::cmp::Ordering::Less => Some(index + 1),
            _ => None,
        })
        .sum();

    Ok(right_order_indexes_sum)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_thirteen_part_two(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, pairs) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;
    let (_, divider_pair) = parse_input("[[2]]\n[[6]]").map_err(|e| Error::Nom(e.to_string()))?;

    let mut packets: Vec<Packet> = pairs
        .into_iter()
        .chain(divider_pair.into_iter())
        .flat_map(|pair| vec![pair.left, pair.right])
        .collect();
    packets.sort();

    Ok(packets
        .iter()
        .enumerate()
        .filter_map(|(index, p)| match p {
            Packet::List(l) => {
                if l.len() == 1 {
                    match &l[0] {
                        Packet::List(l) => {
                            if l.len() == 1 {
                                match &l[0] {
                                    Packet::Number(2 | 6) => Some(index + 1),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        Packet::Number(_) => None,
                    }
                } else {
                    None
                }
            }
            Packet::Number(_) => None,
        })
        .inspect(|p| println!("{p}"))
        .product())
}

#[cfg(test)]
mod tests {
    use crate::{day_thirteen_part_one, day_thirteen_part_two};

    #[test]
    fn day_thirteen_part_one_example() {
        let result = day_thirteen_part_one("example.txt").unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn day_thirteen_part_one_data() {
        let result = day_thirteen_part_one("data.txt").unwrap();
        assert_eq!(result, 6568);
    }

    #[test]
    fn day_thirteen_part_two_example() {
        let result = day_thirteen_part_two("example.txt").unwrap();
        assert_eq!(result, 140);
    }

    #[test]
    fn day_thirteen_part_two_data() {
        let result = day_thirteen_part_two("data.txt").unwrap();
        assert_eq!(result, 19493);
    }
}

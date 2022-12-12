#![allow(clippy::iter_with_drain)]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, multispace1, newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult as NomResult,
};
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug)]
enum Operand {
    Static(u64),
    Previous,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    a: Operand,
    b: Operand,
    operator: Operator,
}

impl Operation {
    fn operate(&self, old: u64) -> u64 {
        match (&self.a, &self.operator, &self.b) {
            (Operand::Static(value_a), Operator::Add, Operand::Static(value_b)) => {
                value_a + value_b
            }
            (Operand::Static(value_a), Operator::Add, Operand::Previous) => value_a + old,
            (Operand::Static(value_a), Operator::Multiply, Operand::Static(value_b)) => {
                value_a * value_b
            }
            (Operand::Static(value_a), Operator::Multiply, Operand::Previous) => value_a * old,
            (Operand::Previous, Operator::Add, Operand::Static(value_b)) => old + value_b,
            (Operand::Previous, Operator::Add, Operand::Previous) => old + old,
            (Operand::Previous, Operator::Multiply, Operand::Static(value_b)) => old * value_b,
            (Operand::Previous, Operator::Multiply, Operand::Previous) => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    operation: Operation,
    divisible_test_factor: u64,
    divisible_test_true_outcome_target: u64,
    divisible_test_false_outcome_target: u64,
}

fn parse_operand(input: &str) -> NomResult<&str, Operand> {
    // old * old
    let (input, operand) = alt((tag("old"), alphanumeric1))(input)?;
    let operand = match operand {
        "old" => Operand::Previous,
        _ => Operand::Static(operand.parse().unwrap_or_default()),
    };
    Ok((input, operand))
}

fn parse_operator(input: &str) -> NomResult<&str, Operator> {
    // old * old
    let (input, operand) = alt((tag("*"), tag("+")))(input)?;
    let operand = match operand {
        "*" => Operator::Multiply,
        _ => Operator::Add,
    };
    Ok((input, operand))
}

fn parse_operation(input: &str) -> NomResult<&str, Operation> {
    // old * old
    let (input, operand_a) = parse_operand(input)?;
    let (input, _) = space1(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, _) = space1(input)?;
    let (input, operand_b) = parse_operand(input)?;

    Ok((
        input,
        Operation {
            a: operand_a,
            b: operand_b,
            operator,
        },
    ))
}

fn parse_monkey(input: &str) -> NomResult<&str, Monkey> {
    let (input, id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items:"),
        separated_list1(tag(","), preceded(space1, complete::u64)),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = preceded(tag("Operation: new = "), parse_operation)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, divisible_test_factor) =
        preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, divisible_test_true_outcome_target) =
        preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, divisible_test_false_outcome_target) =
        preceded(tag("If false: throw to monkey "), complete::u64)(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            divisible_test_factor,
            divisible_test_true_outcome_target,
            divisible_test_false_outcome_target,
        },
    ))
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(newline, parse_monkey)(input)?;

    Ok((input, monkeys))
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_eleven_part_one(path: &str) -> Result<u64, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, monkeys) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut monkey_items: Vec<Vec<u64>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut monkey_inspection_count = vec![0u64; monkeys.len()];

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &monkeys[monkey_index];
            let items: Vec<u64> = monkey_items[monkey_index].drain(0..).collect();
            for item in items {
                let new_worry_level = monkey.operation.operate(item) / 3;

                let target_monkey_index = if new_worry_level % monkey.divisible_test_factor == 0 {
                    monkeys
                        .iter()
                        .position(|m| m.id == monkey.divisible_test_true_outcome_target)
                } else {
                    monkeys
                        .iter()
                        .position(|m| m.id == monkey.divisible_test_false_outcome_target)
                };

                if let Some(index) = target_monkey_index {
                    monkey_items[index].push(new_worry_level);
                }

                monkey_inspection_count[monkey_index] += 1;
            }
        }
    }

    monkey_inspection_count.sort_unstable();

    Ok(monkey_inspection_count.iter().rev().take(2).product())
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_eleven_part_two(path: &str) -> Result<u64, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, monkeys) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut monkey_items: Vec<Vec<u64>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut monkey_inspection_count = vec![0u64; monkeys.len()];

    let common_multiplier: u64 = monkeys.iter().map(|m| m.divisible_test_factor).product();

    for _round in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &monkeys[monkey_index];
            let items: Vec<u64> = monkey_items[monkey_index].drain(0..).collect();
            for item in items {
                let new_worry_level = monkey.operation.operate(item) % common_multiplier;

                let target_monkey_index = if new_worry_level % monkey.divisible_test_factor == 0 {
                    monkeys
                        .iter()
                        .position(|m| m.id == monkey.divisible_test_true_outcome_target)
                } else {
                    monkeys
                        .iter()
                        .position(|m| m.id == monkey.divisible_test_false_outcome_target)
                };

                if let Some(index) = target_monkey_index {
                    monkey_items[index].push(new_worry_level);
                }

                monkey_inspection_count[monkey_index] += 1;
            }
        }
    }

    monkey_inspection_count.sort_unstable();

    Ok(monkey_inspection_count.iter().rev().take(2).product())
}

#[cfg(test)]
mod tests {
    use crate::{day_eleven_part_one, day_eleven_part_two};

    #[test]
    fn day_eleven_part_one_example() {
        let result = day_eleven_part_one("example.txt").unwrap();
        assert_eq!(result, 10605);
    }

    #[test]
    fn day_eleven_part_one_data() {
        let result = day_eleven_part_one("data.txt").unwrap();
        assert_eq!(result, 54036);
    }

    #[test]
    fn day_eleven_part_two_example() {
        let result = day_eleven_part_two("example.txt").unwrap();
        assert_eq!(result, 2_713_310_158);
    }

    #[test]
    fn day_eleven_part_two_data() {
        let result = day_eleven_part_two("data.txt").unwrap();
        assert_eq!(result, 13_237_873_355);
    }
}

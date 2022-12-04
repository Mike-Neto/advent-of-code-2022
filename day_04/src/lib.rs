use std::{fs::read_to_string, io::Error, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult as NomResult,
};

type SectionAssignment = (RangeInclusive<u32>, RangeInclusive<u32>);

fn sections(input: &str) -> NomResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;

    Ok((input, start..=end))
}
fn line(input: &str) -> NomResult<&str, SectionAssignment> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;

    Ok((input, (start, end)))
}
fn section_assignments(input: &str) -> NomResult<&str, Vec<SectionAssignment>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;

    Ok((input, ranges))
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_four_part_one(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path)?;
    let (_, section_assignments) = section_assignments(&input).expect("failed parsing");

    let overlapped_ranges_count = section_assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a = range_a.clone().all(|aa| range_b.contains(&aa));
            let b = range_b.clone().all(|aa| range_a.contains(&aa));
            a || b
        })
        .count();

    Ok(overlapped_ranges_count)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_four_part_two(path: &str) -> Result<usize, Error> {
    let input = read_to_string(path)?;
    let (_, section_assignments) = section_assignments(&input).expect("failed parsing");

    let overlapped_ranges_count = section_assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a = range_a.clone().any(|aa| range_b.contains(&aa));
            let b = range_b.clone().any(|aa| range_a.contains(&aa));
            a || b
        })
        .count();

    Ok(overlapped_ranges_count)
}

#[cfg(test)]
mod tests {
    use crate::{day_four_part_one, day_four_part_two};

    #[test]
    fn day_four_part_one_example() {
        let result = day_four_part_one("example.txt").unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn day_four_part_one_data() {
        let result = day_four_part_one("data.txt").unwrap();
        assert!(result < 396);
        assert_eq!(result, 305);
    }

    #[test]
    fn day_four_part_two_example() {
        let result = day_four_part_two("example.txt").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn day_four_part_two_data() {
        let result = day_four_part_two("data.txt").unwrap();
        assert_eq!(result, 811);
    }
}

use nom::{
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    IResult as NomResult,
};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Vec<u64>>> {
    let (input, groups) =
        separated_list1(multispace1, separated_list1(newline, complete::u64))(input)?;

    Ok((input, groups))
}

/// Finds read file `path` and calculates the sum of all calories for
/// each person, then return the sum based on `window` greatest amounts
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn calc_max_calories_window(path: &str, window: usize) -> Result<u64, Error> {
    let input = std::fs::read_to_string(path).map_err(Error::IO)?;
    let (_, groups) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut max: Vec<u64> = groups.iter().map(|g| g.iter().sum()).collect();
    max.sort_unstable();

    Ok(max.into_iter().rev().take(window).sum())
}

#[cfg(test)]
mod tests {
    use crate::calc_max_calories_window;

    #[test]
    fn day_one_part_one_example() {
        let result = calc_max_calories_window("example.txt", 1).unwrap();
        assert_eq!(result, 24_000);
    }

    #[test]
    fn day_one_part_one_data() {
        let result = calc_max_calories_window("data.txt", 1).unwrap();
        assert_eq!(result, 72_511);
    }

    #[test]
    fn day_one_part_two_example() {
        let result = calc_max_calories_window("example.txt", 3).unwrap();
        assert_eq!(result, 45_000);
    }

    #[test]
    fn day_one_part_two_data() {
        let result = calc_max_calories_window("data.txt", 3).unwrap();
        assert_eq!(result, 212_117);
    }
}

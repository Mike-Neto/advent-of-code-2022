use std::{fs, io::Error};

/// Finds read file `path` and calculates the sum of all calories for
/// each person, then return the sum based on `window` greatest amounts
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn calc_max_calories_window(path: &str, window: usize) -> Result<u64, Error> {
    let mut max: Vec<u64> = fs::read_to_string(path)?
        .split_terminator("\n\n")
        .map(|user_block| {
            let total_calories: u64 = user_block
                .split_terminator('\n')
                .filter_map(|calories| calories.parse::<u64>().ok())
                .sum();

            total_calories
        })
        .collect();

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

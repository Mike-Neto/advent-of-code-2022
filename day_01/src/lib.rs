use std::{fs, io::Error};

pub fn day_one_part_one(path: &str) -> Result<u64, Error> {
    let max = fs::read_to_string(path)?
        .split_terminator("\n\n")
        .map(|user_block| {
            let total_calories: u64 = user_block
                .split_terminator('\n')
                .filter_map(|calories| calories.parse::<u64>().ok())
                .sum();

            total_calories
        })
        .max()
        .unwrap_or(0);

    Ok(max)
}

pub fn day_one_part_two(path: &str) -> Result<u64, Error> {
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

    max.sort();

    Ok(max.into_iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use crate::{day_one_part_one, day_one_part_two};

    #[test]
    fn day_one_part_one_example() {
        let result = day_one_part_one("example.txt").unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn day_one_part_one_data() {
        let result = day_one_part_one("data.txt").unwrap();
        assert_eq!(result, 72511);
    }

    #[test]
    fn day_one_part_two_example() {
        let result = day_one_part_two("example.txt").unwrap();
        assert_eq!(result, 45000);
    }

    #[test]
    fn day_one_part_two_data() {
        let result = day_one_part_two("data.txt").unwrap();
        assert_eq!(result, 212117);
    }
}

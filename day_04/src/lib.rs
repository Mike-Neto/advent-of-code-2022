use std::{fs, io::Error, ops::RangeInclusive};

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_four_part_one(path: &str) -> Result<usize, Error> {
    let overlapped_ranges_count = fs::read_to_string(path)?
        .lines()
        .map(|assignments_pair| {
            let ranges: Vec<RangeInclusive<u64>> = assignments_pair
                .split_terminator(',')
                .take(2)
                .filter_map(|section_interval| {
                    section_interval
                        .split_once('-')
                        .map(|(a, b)| (a.parse().unwrap_or(0)..=b.parse().unwrap_or(0)))
                })
                .collect();
            ranges
        })
        .filter(|ranges| {
            let a = ranges[0].clone().all(|aa| ranges[1].contains(&aa));
            let b = ranges[1].clone().all(|aa| ranges[0].contains(&aa));
            a || b
        })
        .count();

    Ok(overlapped_ranges_count)
}

#[cfg(test)]
mod tests {
    use crate::day_four_part_one;

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
}

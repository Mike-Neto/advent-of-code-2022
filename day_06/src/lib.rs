use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
    Parse(String),
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn find_marker_indexes(path: &str, window_size: usize) -> Result<Vec<usize>, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let lines: Vec<&str> = input.lines().collect();
    let marker_indexes: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            chars
                .windows(window_size)
                .enumerate()
                .find_map(|(index, window)| {
                    if window.iter().all_unique() {
                        Some(index + window_size)
                    } else {
                        None
                    }
                })
        })
        .collect();

    Ok(marker_indexes)
}

#[cfg(test)]
mod tests {
    use crate::find_marker_indexes;

    #[test]
    fn day_six_part_one_example() {
        let result = find_marker_indexes("example.txt", 4).unwrap();
        assert_eq!(result, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn day_six_part_one_data() {
        let result = find_marker_indexes("data.txt", 4).unwrap();
        assert_eq!(result, vec![1542]);
    }

    #[test]
    fn day_six_part_two_example() {
        let result = find_marker_indexes("example.txt", 14).unwrap();
        assert_eq!(result, vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn day_six_part_two_data() {
        let result = find_marker_indexes("data.txt", 14).unwrap();
        assert_eq!(result, vec![3153]);
    }
}

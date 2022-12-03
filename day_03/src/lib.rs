use std::{fs, io::Error};

fn char_to_priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        return u64::from(c) - 96;
    } else {
        u64::from(c) - 38
    }
}

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

pub fn day_three_part_one(path: &str) -> Result<u64, Error> {
    let score: u64 = fs::read_to_string(path)?
        .lines()
        .filter_map(|items| {
            if is_even(items.len())
                && items
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase())
            {
                let half = items.len() / 2;
                let mut first_compartment = items.chars().take(half);
                let second_compartment: Vec<char> = items.chars().skip(half).take(half).collect();
                let mistake = first_compartment.find(|c| second_compartment.contains(c));
                if let Some(mistake) = mistake {
                    return Some(char_to_priority(mistake));
                }
            }
            None
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::day_three_part_one;
    use crate::char_to_priority;

    #[test]
    fn day_three_part_one_example() {
        let result = day_three_part_one("example.txt").unwrap();
        assert_eq!(result, 157);
    }

    #[test]
    fn char_to_priority_works() {
        let result = char_to_priority('a');
        assert_eq!(result, 1);
        let result = char_to_priority('z');
        assert_eq!(result, 26);
        let result = char_to_priority('A');
        assert_eq!(result, 27);
        let result = char_to_priority('Z');
        assert_eq!(result, 52);
    }
}

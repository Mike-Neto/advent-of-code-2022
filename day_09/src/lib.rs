use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Up,
    Left,
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn simulate_rope_motions(path: &str, rope_length: usize) -> Result<usize, std::io::Error> {
    let motions: Vec<(Direction, usize)> = read_to_string(path)?
        .lines()
        .filter_map(|line| {
            if let Some((direction, steps)) = line.split_once(" ") {
                let steps: usize = steps.parse().unwrap_or_default();
                match direction {
                    "U" => return Some((Direction::Up, steps)),
                    "R" => return Some((Direction::Right, steps)),
                    "D" => return Some((Direction::Down, steps)),
                    "L" => return Some((Direction::Left, steps)),
                    _ => return None,
                };
            }
            None
        })
        .collect();

    let mut rope = vec![(0, 0); rope_length];
    let mut playing_field: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);

    for (direction, steps) in motions {
        for _ in 0..steps {
            let head = rope
                .first_mut()
                .expect("should always have at least two elements");
            match direction {
                Direction::Right => {
                    head.0 += 1;
                }
                Direction::Down => {
                    head.1 -= 1;
                }
                Direction::Up => {
                    head.1 += 1;
                }
                Direction::Left => {
                    head.0 -= 1;
                }
            }
            // Update other rope segments
            let head = rope
                .first()
                .expect("should always have at least two elements")
                .clone();
            let last_index = rope.len() - 1;
            let mut previous_segment = head;
            for (index, segment) in rope.iter_mut().enumerate().skip(1) {
                let column_difference = previous_segment.0 - segment.0;
                let row_difference = previous_segment.1 - segment.1;
                let distance: isize = isize::abs(column_difference) + isize::abs(row_difference);

                if isize::abs(column_difference) > 1
                    || isize::abs(row_difference) > 1
                    || distance > 2
                {
                    if column_difference != 0 && row_difference != 0 {
                        segment.0 += if column_difference > 0 { 1 } else { -1 };
                        segment.1 += if row_difference > 0 { 1 } else { -1 };
                    } else if column_difference != 0 {
                        segment.0 += if column_difference > 0 { 1 } else { -1 };
                    } else if row_difference != 0 {
                        segment.1 += if row_difference > 0 { 1 } else { -1 };
                    }

                    if index == last_index {
                        playing_field.insert((segment.0, segment.1));
                    }
                }

                previous_segment = segment.clone();
            }
        }
    }

    Ok(playing_field.len())
}

#[cfg(test)]
mod tests {
    use crate::simulate_rope_motions;

    #[test]
    fn day_nine_part_one_example() {
        let result = simulate_rope_motions("example.txt", 2).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn day_nine_part_one_data() {
        let result = simulate_rope_motions("data.txt", 2).unwrap();
        assert_eq!(result, 6030);
    }

    #[test]
    fn day_nine_part_two_example() {
        let result = simulate_rope_motions("example.txt", 10).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn day_nine_part_two_example_two() {
        let result = simulate_rope_motions("example2.txt", 10).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn day_nine_part_two_data() {
        let result = simulate_rope_motions("data.txt", 10).unwrap();
        assert_eq!(result, 2545);
    }
}

use std::fs::read_to_string;

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_eight_part_one(path: &str) -> Result<usize, std::io::Error> {
    let grid: Vec<Vec<u8>> = read_to_string(path)?
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut interior_visibility_count = 0;
    for (y_index, y) in grid.iter().enumerate().skip(1).rev().skip(1).rev() {
        for (x_index, x) in y.iter().enumerate().skip(1).rev().skip(1).rev() {
            let mut right_visible = true;
            let mut left_visible = true;
            let mut top_visible = true;
            let mut bottom_visible = true;

            for x_index in (0..x_index).rev() {
                let item = grid[y_index][x_index];
                if item >= *x {
                    left_visible = false;
                    break;
                }
            }

            for x_index in (x_index + 1)..y.len() {
                let item = grid[y_index][x_index];
                if item >= *x {
                    right_visible = false;
                    break;
                }
            }

            for y_index in (0..y_index).rev() {
                let item = grid[y_index][x_index];
                if item >= *x {
                    top_visible = false;
                    break;
                }
            }

            for row in grid.iter().skip(y_index + 1) {
                let item = row[x_index];
                if item >= *x {
                    bottom_visible = false;
                    break;
                }
            }

            // Final check
            if right_visible || left_visible || top_visible || bottom_visible {
                interior_visibility_count += 1;
            }
        }
    }

    let visibility_count = interior_visibility_count + ((grid.len() * 2) + (grid[0].len() - 2) * 2);

    Ok(visibility_count)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_eight_part_two(path: &str) -> Result<u64, std::io::Error> {
    let grid: Vec<Vec<u8>> = read_to_string(path)?
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut score_grid = vec![vec![0u64; grid[0].len()]; grid.len()];
    for (y_index, y) in grid.iter().enumerate().skip(1).rev().skip(1).rev() {
        for (x_index, x) in y.iter().enumerate().skip(1).rev().skip(1).rev() {
            let mut right_score = 0;
            let mut left_score = 0;
            let mut top_score = 0;
            let mut bottom_score = 0;

            for x_index in (0..x_index).rev() {
                let item = grid[y_index][x_index];
                left_score += 1;
                if item >= *x {
                    break;
                }
            }

            for x_index in (x_index + 1)..y.len() {
                let item = grid[y_index][x_index];
                right_score += 1;
                if item >= *x {
                    break;
                }
            }

            for y_index in (0..y_index).rev() {
                let item = grid[y_index][x_index];
                top_score += 1;
                if item >= *x {
                    break;
                }
            }

            for row in grid.iter().skip(y_index + 1) {
                let item = row[x_index];
                bottom_score += 1;
                if item >= *x {
                    break;
                }
            }

            score_grid[y_index][x_index] = left_score * right_score * top_score * bottom_score;
        }
    }

    let visibility_count = score_grid.iter().flatten().max().unwrap_or(&0).to_owned();

    Ok(visibility_count)
}

#[cfg(test)]
mod tests {
    use crate::{day_eight_part_one, day_eight_part_two};

    #[test]
    fn day_eight_part_one_example() {
        let result = day_eight_part_one("example.txt").unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn day_eight_part_one_data() {
        let result = day_eight_part_one("data.txt").unwrap();
        assert_eq!(result, 1713);
    }

    #[test]
    fn day_eight_part_two_example() {
        let result = day_eight_part_two("example.txt").unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn day_eight_part_two_data() {
        let result = day_eight_part_two("data.txt").unwrap();
        assert_eq!(result, 268_464);
    }
}

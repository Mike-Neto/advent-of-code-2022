use std::fs::read_to_string;

#[derive(Debug)]
enum Instructions {
    Adddx(isize),
    Noop,
}

impl Instructions {
    fn get_cycle(&self) -> usize {
        match self {
            Instructions::Adddx(_) => 2,
            Instructions::Noop => 1,
        }
    }
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_ten_part_one(path: &str) -> Result<isize, std::io::Error> {
    let instructions: Vec<Instructions> = read_to_string(path)?
        .lines()
        .filter_map(|line| {
            let segments: Vec<&str> = line.split_whitespace().collect();
            if let Some(&instruction_name) = segments.first() {
                match instruction_name {
                    "noop" => return Some(Instructions::Noop),
                    "addx" => {
                        if let Some(value) = segments
                            .last()
                            .map(|value| value.parse().unwrap_or_default())
                        {
                            return Some(Instructions::Adddx(value));
                        }
                    }
                    _ => {}
                }
            }
            None
        })
        .collect();

    let mut cycle_count = 0;
    let mut register = 1;
    let mut signal_stenth = 0;
    for i in &instructions {
        for _ in 0..i.get_cycle() {
            cycle_count += 1;

            if (cycle_count + 20) % 40 == 0 {
                signal_stenth += cycle_count * register;
            }
        }
        match i {
            Instructions::Adddx(value) => register += value,
            Instructions::Noop => {}
        }
    }

    Ok(signal_stenth)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_ten_part_two(path: &str) -> Result<String, std::io::Error> {
    let instructions: Vec<Instructions> = read_to_string(path)?
        .lines()
        .filter_map(|line| {
            let segments: Vec<&str> = line.split_whitespace().collect();
            if let Some(&instruction_name) = segments.first() {
                match instruction_name {
                    "noop" => return Some(Instructions::Noop),
                    "addx" => {
                        if let Some(value) = segments
                            .last()
                            .map(|value| value.parse().unwrap_or_default())
                        {
                            return Some(Instructions::Adddx(value));
                        }
                    }
                    _ => {}
                }
            }
            None
        })
        .collect();

    let mut cycle_count = 0;
    let mut sprite_position: isize = 1;
    let sprite_width = 3;
    let mut screen = vec![vec!['.'; 40]; 6];
    for i in &instructions {
        for _ in 0..i.get_cycle() {
            let x: usize = cycle_count % 40;
            let y: usize = (cycle_count / 40) % 6;

            let sprite_overlaps_current_target = ((sprite_position - 1)
                ..(sprite_position + sprite_width - 1))
                .any(|sprite_x| sprite_x == x.try_into().unwrap());
            if sprite_overlaps_current_target {
                screen[y][x] = '#';
            } else {
                screen[y][x] = '.';
            }

            cycle_count += 1;
        }
        match i {
            Instructions::Adddx(value) => sprite_position += value,
            Instructions::Noop => {}
        }
    }

    println!("{screen:?}");

    Ok(screen.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
    use crate::{day_ten_part_one, day_ten_part_two};

    #[test]
    fn day_nine_part_one_example() {
        let result = day_ten_part_one("example.txt").unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn day_nine_part_one_data() {
        let result = day_ten_part_one("data.txt").unwrap();
        assert_eq!(result, 14920);
    }

    #[test]
    fn day_nine_part_two_example() {
        let result = day_ten_part_two("example.txt").unwrap();
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######....."
        );
    }

    #[test]
    fn day_nine_part_two_data() {
        let result = day_ten_part_two("data.txt").unwrap();
        assert_eq!(
            result,
            "###..#..#..##...##...##..###..#..#.####.#..#.#..#.#..#.#..#.#..#.#..#.#..#....#.###..#..#.#....#..#.#....###..#..#...#..#..#.#..#.#....####.#....#..#.#..#..#...#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....###...##...##..#..#..##..###...##..####."
        );
    }
}

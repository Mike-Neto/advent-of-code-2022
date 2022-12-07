use itertools::Itertools;
use nom::IResult as NomResult;
use std::{collections::BTreeMap, fs::read_to_string};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Debug)]
struct File {
    size: u64,
    name: String,
}

#[derive(Debug)]
enum Command {
    CD(String),
    CDUp,
    LS,
    CDRoot,
}

#[derive(Debug)]
enum Output {
    Command(Command),
    Directory(String),
    File(u64, String),
}

fn parse_input(input: &str) -> NomResult<&str, Vec<Output>> {
    /*
    let (input, lines) = separated_list1(
        newline,
        alt((
            preceded(tag("$ cd "), complete::),
            tag("$ ls"),
            preceded(
                terminated(alt((alphanumeric1, tag("dir"))), tag(" ")),
                alphanumeric1,
            ),
        )),
    )(input)?;
    */

    let commands = input
        .lines()
        .filter_map(|line| {
            let segments: Vec<&str> = line.split_whitespace().collect();
            if segments.len() >= 1 {
                let is_command = segments[0] == "$";
                if is_command {
                    if segments.len() == 3 && segments[1] == "cd" {
                        match segments[2] {
                            ".." => return Some(Output::Command(Command::CDUp)),
                            "/" => return Some(Output::Command(Command::CDRoot)),
                            _ => {
                                return Some(Output::Command(Command::CD(segments[2].to_string())))
                            }
                        }
                    }
                    if segments.len() == 2 && segments[1] == "ls" {
                        return Some(Output::Command(Command::LS));
                    }
                } else {
                    if segments.len() == 2 {
                        match segments[0] {
                            "dir" => return Some(Output::Directory(segments[1].to_string())),
                            _ => {
                                return Some(Output::File(
                                    segments[0].parse().unwrap_or_default(),
                                    segments[1].to_string(),
                                ))
                            }
                        }
                    }
                }
            }
            None
        })
        .collect();

    Ok((input, commands))
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_seven_part_one(path: &str) -> Result<u64, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, output) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<String> = vec![];

    for o in output.into_iter() {
        match o {
            Output::Command(c) => match c {
                Command::LS => {
                    directories
                        .entry(
                            context
                                .iter()
                                .cloned()
                                .intersperse("/".to_string())
                                .collect::<String>(),
                        )
                        .or_insert(vec![]);
                }
                Command::CDUp => {
                    context.pop();
                }
                Command::CDRoot => {
                    context.push("".to_string());
                }
                Command::CD(dir_name) => {
                    context.push(dir_name);
                }
            },
            Output::Directory(_) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/".to_string())
                            .collect::<String>(),
                    )
                    .or_insert(vec![]);
            }
            Output::File(size, name) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/".to_string())
                            .collect::<String>(),
                    )
                    .and_modify(|vec| {
                        vec.push(File { name, size });
                    });
            }
        }
    }

    let mut sizes: BTreeMap<String, u64> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files.iter().map(|File { size, .. }| size).sum();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }
    let sum = sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum();

    Ok(sum)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_seven_part_two(path: &str) -> Result<u64, Error> {
    let input = read_to_string(path).map_err(Error::IO)?;
    let (_, output) = parse_input(&input).map_err(|e| Error::Nom(e.to_string()))?;

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<String> = vec![];

    for o in output.into_iter() {
        match o {
            Output::Command(c) => match c {
                Command::LS => {
                    directories
                        .entry(
                            context
                                .iter()
                                .cloned()
                                .intersperse("/".to_string())
                                .collect::<String>(),
                        )
                        .or_insert(vec![]);
                }
                Command::CDRoot => {
                    context.push("".to_string());
                }
                Command::CDUp => {
                    context.pop();
                }
                Command::CD(dir_name) => {
                    context.push(dir_name);
                }
            },
            Output::Directory(_) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/".to_string())
                            .collect::<String>(),
                    )
                    .or_insert(vec![]);
            }
            Output::File(size, name) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/".to_string())
                            .collect::<String>(),
                    )
                    .and_modify(|vec| {
                        vec.push(File { name, size });
                    });
            }
        }
    }

    let mut sizes: BTreeMap<String, u64> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files.iter().map(|File { size, .. }| size).sum();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get("").unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size.clone())
        .collect::<Vec<u64>>();

    valid_dirs.sort_unstable();

    Ok(valid_dirs[0])
}

#[cfg(test)]
mod tests {
    use crate::{day_seven_part_one, day_seven_part_two};

    #[test]
    fn day_six_part_one_example() {
        let result = day_seven_part_one("example.txt").unwrap();
        assert_eq!(result, 95_437);
    }

    #[test]
    fn day_six_part_one_data() {
        let result = day_seven_part_one("data.txt").unwrap();
        assert_eq!(result, 1611443);
    }

    #[test]
    fn day_six_part_two_example() {
        let result = day_seven_part_two("example.txt").unwrap();
        assert_eq!(result, 24_933_642);
    }

    #[test]
    fn day_six_part_two_data() {
        let result = day_seven_part_two("data.txt").unwrap();
        assert_eq!(result, 1);
    }
}

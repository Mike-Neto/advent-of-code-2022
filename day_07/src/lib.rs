use std::{cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Nom(String),
}

#[derive(Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn parse_input<'a>(input: &'a str, root: &'a mut Rc<Dir>) {
    let mut cwd = Rc::clone(root);
    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words.get(2) {
                Some(&"/") => cwd = Rc::clone(root),
                Some(&"..") => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                Some(&dirname) => {
                    let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                    cwd = newdir;
                }
                _ => {}
            },
            ("dir", dirname) => {
                cwd.subdir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_seven_part_one(path: &str) -> Result<usize, Error> {
    let mut root = Rc::new(Dir::default());
    let input = read_to_string(path).map_err(Error::IO)?;
    parse_input(&input, &mut root);

    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size <= 100_000 {
            total += size;
        }
    }

    Ok(total)
}

/// TODO
///
/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it.
pub fn day_seven_part_two(path: &str) -> Result<usize, Error> {
    let mut root = Rc::new(Dir::default());
    let input = read_to_string(path).map_err(Error::IO)?;
    parse_input(&input, &mut root);

    let total_size = root.get_size();
    let free_space = 70_000_000 - total_size;
    let space_needed = 30_000_000 - free_space;

    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = usize::MAX;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size);
        }
    }
    Ok(best)
}

#[cfg(test)]
mod tests {
    use crate::{day_seven_part_one, day_seven_part_two};

    #[test]
    fn day_seven_part_one_example() {
        let result = day_seven_part_one("example.txt").unwrap();
        assert_eq!(result, 95_437);
    }

    #[test]
    fn day_seven_part_one_data() {
        let result = day_seven_part_one("data.txt").unwrap();
        assert_eq!(result, 1_611_443);
    }

    #[test]
    fn day_seven_part_two_example() {
        let result = day_seven_part_two("example.txt").unwrap();
        assert_eq!(result, 24_933_642);
    }

    #[test]
    fn day_seven_part_two_data() {
        let result = day_seven_part_two("data.txt").unwrap();
        assert_eq!(result, 2_086_088);
    }
}

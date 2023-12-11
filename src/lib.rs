use curl::easy::{Easy, HttpVersion};
use std::fs;
use std::path::PathBuf;
use std::fmt;

pub fn get_input(year: &str, day: &str) -> String {
    let mut input = String::new();

    // cache input
    let path = PathBuf::from(format!("./.cache/day{day}.input"));

    if path.exists() {
        input = fs::read_to_string(path).unwrap();
    } else {
        let mut easy = Easy::new();
        easy.cookie(&format!(
            "session={}",
            fs::read_to_string(".session")
                .unwrap_or_else(|_| panic!("You must put your session key in '.session'"))
        ))
        .unwrap();
        // Use http/1 not http/2, for some reason it is buggy otherwise
        easy.http_version(HttpVersion::V11).unwrap();

        easy.url(&format!("https://adventofcode.com/{year}/day/{day}/input"))
            .unwrap();

        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    input.extend(data.iter().map(|b| *b as char));
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        fs::write(&path, &input).unwrap();
    }

    input
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Pos {
    fn from(f: (isize, isize)) -> Pos {
        Pos { x: f.0, y: f.1 }
    }
}

struct Map<T> {
    content: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: TryFrom<char>> Map<T> {
    fn from_str(s: &str) -> Map<T> {
        let mut l = s.lines();
        let width = l.next().unwrap().len();
        let height = l.count() + 1;

        let content = s
            .replace("\r", "")
            .chars()
            .filter_map(|c| T::try_from(c).ok())
            .collect();

        Map {
            content,
            width,
            height,
        }
    }
}

impl<T> Map<T> {
    fn get_index(&self, pos: &Pos) -> Option<usize> {
        if pos.x < self.width as isize && pos.x >= 0 && pos.y < self.height as isize && pos.y >= 0 {
            return Some(pos.x as usize + pos.y as usize * self.width);
        }
        None
    }

    fn get(&self, pos: &Pos) -> Option<&T> {
        self.content.get(self.get_index(pos)?)
    }

    fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        let index = self.get_index(pos)?;
        self.content.get_mut(index)
    }
}

impl<T: fmt::Display> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                write!(f, "{}", self.get(&(x, y).into()).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            get_input("2023", "1"),
            fs::read_to_string("inputs/day01").unwrap()
        );
    }
}

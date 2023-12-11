use rustvent2023::get_input;
use std::convert::{From, Into, TryFrom};
use std::fmt;
use std::time;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Tile::*;
        match self {
            Empty => write!(f, " "),
            Start => write!(f, "S"),
            Horizontal => write!(f, "━"),
            Vertical => write!(f, "┃"),
            NorthEast => write!(f, "┗"),
            NorthWest => write!(f, "┛"),
            SouthEast => write!(f, "┏"),
            SouthWest => write!(f, "┓"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    North,
    West,
    South,
    East,
}

impl Dir {
    fn opp(&self) -> Dir {
        use Dir::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Tile::*;
        match c {
            '.' => Ok(Empty),
            'S' => Ok(Start),
            '-' => Ok(Horizontal),
            '|' => Ok(Vertical),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn get_dirs(&self) -> Option<(Dir, Dir)> {
        use Dir::*;
        use Tile::*;

        match self {
            Horizontal => Some((East, West)),
            Vertical => Some((North, South)),
            NorthEast => Some((North, East)),
            NorthWest => Some((North, West)),
            SouthEast => Some((South, East)),
            SouthWest => Some((South, West)),
            _ => None,
        }
    }

    fn goes_to(&self, dir: Dir) -> Option<Dir> {
        match self.get_dirs() {
            Some((dir1, dir2)) => {
                if dir.opp() == dir1 {
                    Some(dir2)
                } else if dir.opp() == dir2 {
                    Some(dir1)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
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

struct Field {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    start: Pos,
}

impl Pos {
    fn go(&mut self, dir: &Dir) -> &mut Pos {
        use Dir::*;
        match dir {
            North => self.y -= 1,
            East => self.x += 1,
            West => self.x -= 1,
            South => self.y += 1,
        }

        self
    }
}

impl Field {
    fn from_str(s: &str) -> Field {
        let mut l = s.lines();
        let width = l.next().unwrap().len();
        let height = l.count() + 1;

        let tiles: Vec<Tile> = s
            .replace("\r", "")
            .chars()
            .filter_map(|c| Tile::try_from(c).ok())
            .collect();

        let idx = tiles.iter().position(|t| *t == Tile::Start).unwrap();
        let start = Pos {
            x: (idx % width) as isize,
            y: (idx / width) as isize,
        };

        Field {
            tiles,
            width,
            height,
            start,
        }
    }

    fn get(&self, pos: &Pos) -> Option<&Tile> {
        if pos.x < self.width as isize && pos.x >= 0 && pos.y < self.height as isize && pos.y >= 0 {
            return self.tiles.get(pos.x as usize + pos.y as usize * self.width);
        }
        None
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                write!(f, "{}", self.get(&(x, y).into()).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part_one(field: &Field) -> usize {
    use Dir::*;
    for s in [North, East, South, West] {
        let mut pos = field.start.clone();
        let mut dir = s;

        let mut n = 0;
        loop {
            pos.go(&dir);
            n += 1;

            dir = match field.get(&pos) {
                Some(tile) => match tile {
                    Tile::Start => return n / 2,
                    _ => match tile.goes_to(dir) {
                        Some(dir) => dir,
                        None => break,
                    },
                },
                None => break,
            };
        }
    }
    panic!("Did not find any loop");
}

fn part_two(field: &Field) -> usize {
    use Dir::*;
    let mut loo: Vec<Pos> = Vec::new();
    let mut start_emit = false;

    'outer: for s in [North, East, South, West] {
        let mut pos = field.start.clone();
        let mut dir = s.clone();
        loo.clear();

        loop {
            pos.go(&dir);
            loo.push(pos.clone());

            dir = match field.get(&pos) {
                Some(tile) => match tile {
                    Tile::Start => {
                        start_emit = !((s == dir) && (s == East || s == West));
                        break 'outer;
                    }
                    _ => match tile.goes_to(dir) {
                        Some(dir) => dir,
                        None => break,
                    },
                },
                None => break,
            };
        }
    }

    let inner: Vec<bool> = vec![false; field.tiles.len()];
    let mut m = Map {
        content: inner,
        width: field.width,
        height: field.height,
    };

    'pos: for pos in &loo {
        let tile1 = field.get(&pos).unwrap();
        if !match tile1 {
            Tile::Horizontal => false,
            Tile::Empty => panic!("Empty tile can't be part of loop"),
            Tile::Start => start_emit,
            _ => true,
        } {
            continue;
        }

        let on_border = (*tile1 == Tile::NorthEast) || (*tile1 == Tile::SouthEast);
        for x in pos.x + 1..field.width as isize {
            let new_pos = (x, pos.y).into();
            if on_border {
                let tile2 = field.get(&new_pos).unwrap();
                if (*tile2 == Tile::NorthWest && *tile1 == Tile::SouthEast) || (*tile2 == Tile::SouthWest && *tile1 == Tile::NorthEast) {
                    continue 'pos;
                }
            }
            let p = m.get_mut(&new_pos).unwrap();
            *p = !*p;
        }
    }

    for pos in &loo {
        let p = m.get_mut(&pos).unwrap();
        *p = false;
    }

    m.content.iter().filter(|&b| *b).count()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "10");
    let field = Field::from_str(&input);

    let now = time::Instant::now();
    let sol_p1 = part_one(&field);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let now = time::Instant::now();
    let sol_p2 = part_two(&field);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    const TEST2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    const TEST3: &str = 
"............
.S-------7..
.|F-----7|..
.||.....||..
FJ|.....|L7.
|.|...F-J.|.
|.L-7.|...|.
L---J.L---J.
............
";

    #[test]
    fn test_parse() {
        let field = Field::from_str(TEST);

        assert_eq!(field.width, 5);
        assert_eq!(field.height, 5);
        assert_eq!(
            field.get(&(3isize, 1isize).into()).unwrap(),
            &Tile::SouthWest
        );
    }

    #[test]
    fn test_part_one() {
        let field = Field::from_str(TEST);
        println!("{}", field);
        assert_eq!(part_one(&field), 4);
    }

    #[test]
    fn test_part_two() {
        let field = Field::from_str(TEST2);
        assert_eq!(part_two(&field), 10);
    }

    #[test]
    fn test_part_two_easy() {
        let field = Field::from_str(TEST);
        assert_eq!(part_two(&field), 1);
    }

    #[test]
    fn test_part_two_medium() {
        let field = Field::from_str(TEST3);
        assert_eq!(part_two(&field), 6);
    }
}

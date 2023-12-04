use lazy_static::lazy_static;
use regex::Regex;
use rustvent2023::get_input;
use std::collections::HashMap;
use std::time;

const NONSYMB: &str = ".01234567890";

fn issymb(c: char) -> bool {
    !NONSYMB.contains(c)
}

fn part_one(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let lines: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();

    let mut n = 0;

    for (y, line) in input.lines().enumerate() {
        'nums: for cap in RE.captures_iter(line) {
            let num = cap[0].parse::<usize>().unwrap();
            let range = cap.get(0).unwrap().range();

            if range.start > 0 {
                if issymb(lines[y][range.start - 1]) {
                    n += num;
                    continue 'nums;
                }
                if y > 0 && issymb(lines[y - 1][range.start - 1]) {
                    n += num;
                    continue 'nums;
                }
                if y < lines.len() - 1 && issymb(lines[y + 1][range.start - 1]) {
                    n += num;
                    continue 'nums;
                }
            }

            if range.end < lines[y].len() - 1 {
                if issymb(lines[y][range.end]) {
                    n += num;
                    continue 'nums;
                }
                if y > 0 && issymb(lines[y - 1][range.end]) {
                    n += num;
                    continue 'nums;
                }
                if y < lines.len() - 1 && issymb(lines[y + 1][range.end]) {
                    n += num;
                    continue 'nums;
                }
            }

            for x in range {
                if y > 0 && issymb(lines[y - 1][x]) {
                    n += num;
                    continue 'nums;
                }
                if y < lines.len() - 1 && issymb(lines[y + 1][x]) {
                    n += num;
                    continue 'nums;
                }
            }
        }
    }

    n
}

fn part_two(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let lines: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();

    let mut n = 0;

    let mut gears: HashMap<(usize, usize), usize> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        'nums: for cap in RE.captures_iter(line) {
            let num = cap[0].parse::<usize>().unwrap();
            let range = cap.get(0).unwrap().range();

            if range.start > 0 {
                if '*' == (lines[y][range.start - 1]) {
                    let idx = (y, range.start - 1);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
                if y > 0 && '*' == (lines[y - 1][range.start - 1]) {
                    let idx = (y - 1, range.start - 1);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
                if y < lines.len() - 1 && '*' == (lines[y + 1][range.start - 1]) {
                    let idx = (y + 1, range.start - 1);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
            }

            if range.end < lines[y].len() - 1 {
                if '*' == (lines[y][range.end]) {
                    let idx = (y, range.end);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
                if y > 0 && '*' == (lines[y - 1][range.end]) {
                    let idx = (y - 1, range.end);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
                if y < lines.len() - 1 && '*' == (lines[y + 1][range.end]) {
                    let idx = (y + 1, range.end);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
            }

            for x in range {
                if y > 0 && '*' == (lines[y - 1][x]) {
                    let idx = (y - 1, x);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
                if y < lines.len() - 1 && '*' == (lines[y + 1][x]) {
                    let idx = (y + 1, x);
                    match gears.get(&idx) {
                        Some(val) => n += num * val,
                        None => {
                            gears.insert(idx, num);
                        }
                    }
                    continue 'nums;
                }
            }
        }
    }

    n
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = &get_input("2023", "3");

    let now = time::Instant::now();
    let sol_p1 = part_one(&input);
    println!(
        "Solution part one: {sol_p1} took: {}s",
        now.elapsed().as_secs_f32()
    );

    let now = time::Instant::now();
    let sol_p2 = part_two(&input);
    println!(
        "Solution part two: {sol_p2} took: {}s",
        now.elapsed().as_secs_f32()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST), 467835);
    }
}

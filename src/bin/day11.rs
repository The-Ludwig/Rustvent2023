use rustvent2023::{get_input, Pos};
use std::convert::{From, Into, TryFrom};
use std::fmt;
use std::time;

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .enumerate()
        .map(|l| {
            l.1.chars()
                .enumerate()
                .filter(|c| c.1 == '#')
                .map(move |c| (c.0 as isize, l.0 as isize).into())
        })
        .flatten()
        .collect()
}

fn part_n(gals: &Vec<Pos>, expand: isize) -> usize {
    let mut expand_x: Vec<_> = (0..gals.iter().map(|e| e.x).max().unwrap()).collect();
    let mut expand_y: Vec<_> = (0..gals.iter().map(|e| e.y).max().unwrap()).collect();
    expand_x.reverse();
    expand_y.reverse();

    for gal in gals {
        expand_x.retain(|&x| x != gal.x);
        expand_y.retain(|&y| y != gal.y);
    }

    let mut expanded = gals.clone();

    for x in expand_x {
        for gal in &mut expanded {
            if gal.x > x {
                gal.x += expand-1;
            }
        }
    }

    for y in expand_y {
        for gal in &mut expanded {
            if gal.y > y {
                gal.y += expand-1;
            }
        }
    }

    let mut dists: Vec<usize> = Vec::new();

    for i in 0..expanded.len() - 1 {
        for j in i + 1..expanded.len() {
            dists.push((expanded[i].x - expanded[j].x).abs() as usize + (expanded[i].y - expanded[j].y).abs() as usize);
        }
    }

    dists.iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "11");
    let field = parse(&input);

    let now = time::Instant::now();
    let sol_p1 = part_n(&field, 2);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let now = time::Instant::now();
    let sol_p2 = part_n(&field, 1000000);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_parse() {
        let gals = parse(TEST);
        println!("{:?}", gals);
        assert!(gals.contains(&(3, 0).into()));
    }

    #[test]
    fn test_part_one() {
        let gals = parse(TEST);
        assert_eq!(374, part_n(&gals, 2));
    }

    #[test]
    fn test_part_two() {
        let gals = parse(TEST);
        assert_eq!(8410, part_n(&gals, 100));
    }
}

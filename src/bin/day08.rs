use lazy_static::lazy_static;
use regex::Regex;
use rustvent2023::get_input;
use std::collections::HashMap;
use std::{time, usize};

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    L,
    R,
}

fn parse(input: &str) -> (Vec<Dir>, Map) {
    let mut parts = input.split("\n\n");

    let dirs = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => panic!("Not a valid direction"),
        })
        .collect();

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?<from>[A-Z1-9]{3}) = \((?<left>[A-Z1-9]{3}), (?<right>[A-Z1-9]{3})\)")
                .unwrap();
    }

    let map = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let ma = RE.captures(l).unwrap();
            (
                ma.name("from").unwrap().as_str(),
                (
                    ma.name("left").unwrap().as_str(),
                    ma.name("right").unwrap().as_str(),
                ),
            )
        })
        .collect();

    (dirs, map)
}

fn part_one(dirs: &Vec<Dir>, map: &Map) -> usize {
    let mut loc = "AAA";

    let mut n = 0;
    'outer: loop {
        for dir in dirs {
            loc = match dir {
                Dir::L => map[loc].0,
                Dir::R => map[loc].1,
            };
            n += 1;
            if loc == "ZZZ" {
                break 'outer;
            }
        }
    }
    n
}

fn part_two(dirs: &Vec<Dir>, map: &Map) -> usize {
    let mut locs: Vec<_> = map
        .keys()
        .filter(|c| c.chars().nth(2).unwrap() == 'A')
        .map(|c| *c)
        .collect();

    let rep = 5000;

    let finals: HashMap<&str, (Vec<usize>, &str)> = map
        .keys()
        .map(|m| {
            let mut fins = Vec::new();

            let mut loc = *m;

            let mut n = 0;
            for _ in 0..rep {
                for dir in dirs {
                    loc = match dir {
                        Dir::L => map[loc].0,
                        Dir::R => map[loc].1,
                    };
                    if loc.chars().nth(2).unwrap() == 'Z' {
                        fins.push(n);
                    }
                    n += 1;
                }
            }

            (*m, (fins, loc))
        })
        .collect();

    let mut n = 0;
    let mut found = false;
    while !found {
        for fin in &finals[locs[0]].0 {
            found = true;
            for loc in locs.iter().skip(1) {
                if !finals[loc].0.contains(&fin) {
                    found = false;
                    break;
                }
            }
            if found {
                n += fin + 1;
                break;
            }
        }

        if !found {
            for loc in &mut locs {
                *loc = finals[loc].1;
            }
            n += dirs.len() * rep;
        }
        print!("{}\r", n);
    }
    n
}

fn euclid(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / euclid(a, b) * b
}

fn part_two_way2(dirs: &Vec<Dir>, map: &Map) -> usize {
    let locs: Vec<_> = map
        .keys()
        .filter(|c| c.chars().nth(2).unwrap() == 'A')
        .map(|c| *c)
        .collect();

    let paths: HashMap<&str, (usize, usize)> = locs
        .iter() // (cycle_len, cycle_idx)
        .map(|m| {
            let mut loc = *m;
            let mut visited = Vec::new();

            let mut n = 0;
            loop {
                for (i, dir) in dirs.iter().enumerate() {
                    visited.push((loc, i));
                    n += 1;
                    loc = match dir {
                        Dir::L => map[loc].0,
                        Dir::R => map[loc].1,
                    };
                    if let Some(i) = visited.iter().position(|&r| r == (loc, i + 1)) {
                        return (*m, (n, i));
                    }
                }
            }
        })
        .collect();

    println!("{:#?}", paths);

    let m: Vec<usize> = paths.values().map(|(v1, v2)| v1 - v2).collect();
    // Least common multiple
    let mut lcm_ = m[0];
    for mi in m {
        lcm_ = lcm(lcm_, mi);
    }

    lcm_
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "8");
    let (dirs, map) = parse(&input);

    let now = time::Instant::now();
    let sol_p1 = part_one(&dirs, &map);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let now = time::Instant::now();
    let sol_p2 = part_two_way2(&dirs, &map);
    println!(
        "Solution part two in a smarter way: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    return Ok(());

    let now = time::Instant::now();
    let sol_p2 = part_two(&dirs, &map);
    println!(
        "Solution part two: {sol_p2} took: {}s",
        now.elapsed().as_secs()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const TEST2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const TEST3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_parse() {
        use Dir::*;
        let (dirs, map) = parse(TEST);
        assert_eq!(dirs, vec![R, L]);
        assert_eq!(map["CCC"], ("ZZZ", "GGG"));
    }

    #[test]
    fn test_part_one() {
        let (dirs, map) = parse(TEST);
        println!("{:#?}", map);
        assert_eq!(part_one(&dirs, &map), 2);

        let (dirs, map) = parse(TEST2);
        assert_eq!(part_one(&dirs, &map), 6);
    }

    #[test]
    fn test_part_two() {
        let (dirs, map) = parse(TEST3);
        assert_eq!(part_two(&dirs, &map), 6);
    }

    #[test]
    fn test_part_two_smarter() {
        let (dirs, map) = parse(TEST3);
        assert_eq!(part_two_way2(&dirs, &map), 6);
    }
}

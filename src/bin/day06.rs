use rustvent2023::get_input;
use std::{f64, time};

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut l = input.lines();

    (
        l.next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        l.next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
    )
}

fn parse_part2(input: &str) -> (usize, usize) {
    let mut l = input.lines();

    (
        l.next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .replace(" ", "")
            .parse()
            .unwrap(),
        l.next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(" ", "")
            .parse()
            .unwrap(),
    )
}

fn part_one(time: &Vec<usize>, dist: &Vec<usize>) -> usize {
    time.iter()
        .zip(dist.iter())
        .map(|(t, d)| {
            let mut n = 0;
            for h in 1..*t {
                if h * (t - h) > *d {
                    n += 1;
                }
            }
            n
        })
        .product()
}

// turns out, this problem has an analytical solution
// quadratic formulas rock
fn part_two(time: usize, dist: usize) -> usize {
    2 * ((time as f64 * time as f64 / 4.0 - dist as f64).sqrt() as usize)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "6");
    let (time, dist) = parse(&input);

    let now = time::Instant::now();
    let sol_p1 = part_one(&time, &dist);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let (time, dist) = parse_part2(&input);
    let now = time::Instant::now();
    let sol_p2 = part_two(time, dist);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let (time, dist) = parse(TEST);
        assert_eq!(time, vec![7, 15, 30]);
        assert_eq!(dist, vec![9, 40, 200]);
    }

    #[test]
    fn test_parse_part2() {
        let (time, dist) = parse_part2(TEST);
        assert_eq!(time, 71530);
        assert_eq!(dist, 940200);
    }

    #[test]
    fn test_part_one() {
        let (time, dist) = parse(TEST);
        assert_eq!(part_one(&time, &dist), 288);
    }

    #[test]
    fn test_part_two() {
        let (time, dist) = parse_part2(TEST);
        assert_eq!(part_two(time, dist), 71503);
    }
}

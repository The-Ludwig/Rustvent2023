use lazy_static::lazy_static;
use regex::Regex;
use rustvent2023::get_input;
use std::error::Error;
use std::str::FromStr;
use std::time;
use std::cmp::min;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Card +(?<id>\d+): (?<winning>[\d ]+) \| (?<numbers>[\d ]+)").unwrap();
        }

        let cap = RE.captures(s).ok_or("Not a valid card description.")?;

        let id = cap.name("id").unwrap().as_str().parse().unwrap();
        let winning = cap
            .name("winning")
            .unwrap()
            .as_str()
            .trim()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let numbers = cap
            .name("numbers")
            .unwrap()
            .as_str()
            .trim()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Card {
            id,
            winning,
            numbers,
        })
    }
}

fn parse(input: &str) -> Vec<Card> {
    input.lines().map(|l| Card::from_str(l).unwrap()).collect()
}

fn part_one(input: &Vec<Card>) -> usize {
    input
        .iter()
        .map(|card| {
            card.numbers
                .iter()
                .filter(|n| card.winning.contains(n))
                .fold(1, |acc, _x| acc * 2)/2
        })
        .sum()
}

fn part_two(input: &Vec<Card>) -> usize {
    let mut instances = vec![1; input.len()];
    for (i, card) in input.iter().enumerate() {
        let wins = card.numbers
            .iter()
            .filter(|n| card.winning.contains(n))
            .count();

        for j in min(i+1, instances.len())..min(i+wins+1, instances.len()) {
            instances[j] += instances[i];
        }
    }

    instances.iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2023", "4"));

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

    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST)[0],
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn test_part_one() {
        let input = parse(TEST);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = parse(TEST);
        assert_eq!(part_two(&input), 30);
    }
}

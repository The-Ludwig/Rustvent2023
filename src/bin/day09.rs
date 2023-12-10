use lazy_static::lazy_static;
use regex::Regex;
use rustvent2023::get_input;
use std::collections::HashMap;
use std::{time, usize};


fn parse(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|l| l.split_whitespace().map(str::parse).collect()).collect::<Result<_, _>>().unwrap()
}

fn part_one(nums: &Vec<Vec<isize>>) -> isize {
    nums.iter().map(|ns| {
        let mut vec = ns.clone();
        let mut last = Vec::new();
        while !vec.iter().all(|&n| n == 0){
            last.push(*vec.last().unwrap());
            vec = vec.windows(2).map(|w| w[1]-w[0]).collect();
        }
        last.iter().sum::<isize>()
    }).sum()
}

fn part_two(nums: &Vec<Vec<isize>>) -> isize {
    nums.iter().map(|ns| {
        let mut vec = ns.clone();
        let mut first = Vec::new();
        while !vec.iter().all(|&n| n == 0){
            first.push(*vec.first().unwrap());
            vec = vec.windows(2).map(|w| w[1]-w[0]).collect();
        }
        first.iter().rev().fold(0, |acc, el| el-acc)
    }).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "9");
    let nums = parse(&input);

    let now = time::Instant::now();
    let sol_p1 = part_one(&nums);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let now = time::Instant::now();
    let sol_p2 = part_two(&nums);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_parse() {
        let nums = parse(TEST);
        assert_eq!(nums[1], vec![1, 3, 6, 10, 15, 21]);
    }

    #[test]
    fn test_part_one() {
        let nums = parse(TEST);
        assert_eq!(part_one(&nums), 114);
    }

    #[test]
    fn test_part_two() {
        let nums = parse(TEST);
        assert_eq!(part_two(&nums), 2);
    }
}

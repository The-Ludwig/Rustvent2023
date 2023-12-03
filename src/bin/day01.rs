use rustvent2023::get_input;
use std::time;

const NUM_2_DIG: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|m: &str| {
            let mut cs = m.chars().filter(|c| '0' <= *c && *c <= '9');
            let first = cs.next().unwrap();
            let last = cs.last().unwrap_or(first);
            let s = format!("{first}{last}");
            s.parse::<usize>().unwrap()
        })
    .sum()
}

fn part_two(input: &str) -> usize {
    let mut replaced = input.to_string();


    // this can't be any more ugly
    for (word, dig) in NUM_2_DIG {
        replaced = replaced.replace(word, &format!("{word}{dig}{word}"));
    }

    part_one(&replaced)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "1");

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

    const TEST: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2), 281);
    }
}

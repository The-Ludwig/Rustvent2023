use lazy_static::lazy_static;
use regex::Regex;
use rustvent2023::get_input;
use std::cmp::max;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::time;

// #[derive(Debug)]
// enum Cubes {
//     Red,
//     Green,
//     Blue,
// }
//

#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    nred: usize,
    ngreen: usize,
    nblue: usize,
}

impl FromStr for Outcome {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<number>\d+) (?<color>red|blue|green)",).unwrap();
        }

        let cap = RE.captures(s).ok_or("Not a valid box description")?;

        let (mut nred, mut ngreen, mut nblue) = (0, 0, 0);

        for b in RE.captures_iter(s) {
            let num: usize = b
                .name("number")
                .ok_or("no number match")?
                .as_str()
                .parse()?;
            match b.name("color").ok_or("no color match")?.as_str() {
                "red" => nred += num,
                "blue" => nblue += num,
                "green" => ngreen += num,
                _ => Err("Nooo")?,
            };
        }

        Ok(Self {
            nred,
            ngreen,
            nblue,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    outcomes: Vec<Outcome>,
}

impl FromStr for Game {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Game (?<gn>\d+):(?<results>(( \d+ (red|blue|green),?)+;?)+)",)
                    .unwrap();
        }

        let cap = RE.captures(s).ok_or("Not a valid game description")?;

        let id = cap
            .name("gn")
            .ok_or("No game number found")?
            .as_str()
            .parse::<usize>()?;

        Ok(Self {
            id,
            outcomes: cap
                .name("results")
                .ok_or("Wrong formatting")?
                .as_str()
                .split(";")
                .map(|outcome| Outcome::from_str(outcome).unwrap())
                .collect(),
        })
    }
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::from_str(l).unwrap()).collect()
}

fn part_one(input: &Vec<Game>) -> usize {
    let mut n = 0;

    'outer: for game in input {
        for outcome in &game.outcomes {
            if outcome.nred > 12 || outcome.ngreen > 13 || outcome.nblue > 14 {
                continue 'outer;
            }
        }
        n += game.id;
    }

    n
}

fn part_two(input: &Vec<Game>) -> usize {
    let mut n = 0;

    for game in input {
        let (mut nred, mut ngreen, mut nblue) = (0, 0, 0);
        for outcome in &game.outcomes {
            nred = max(nred, outcome.nred);
            ngreen = max(ngreen, outcome.ngreen);
            nblue = max(nblue, outcome.nblue);
        }

        n += nred * ngreen * nblue;
    }

    n
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse(&get_input("2023", "2"));

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

    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST)[0],
            Game {
                id: 1,
                outcomes: vec![
                    Outcome {
                        nred: 4,
                        ngreen: 0,
                        nblue: 3
                    },
                    Outcome {
                        nred: 1,
                        ngreen: 2,
                        nblue: 6
                    },
                    Outcome {
                        nred: 0,
                        ngreen: 2,
                        nblue: 0
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part_one() {
        let game = parse(TEST);
        assert_eq!(part_one(&game), 8);
    }

    #[test]
    fn test_part_two() {
        let game = parse(TEST);
        assert_eq!(part_two(&game), 2286);
    }
}

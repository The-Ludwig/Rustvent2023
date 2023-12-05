use rustvent2023::get_input;
use std::error::Error;
use std::ops::{Range, RangeBounds};
use std::str::FromStr;
use std::time;

fn containment<Idx: PartialOrd + Copy>(
    inner: &Range<Idx>,
    bounding: &Range<Idx>,
) -> (Option<Range<Idx>>, Vec<Range<Idx>>) {
    if (inner.start < bounding.start) & (inner.end > bounding.end) {
        (
            Some(bounding.clone()),
            vec![
                Range {
                    start: inner.start,
                    end: bounding.start,
                },
                Range {
                    start: bounding.end,
                    end: inner.end,
                },
            ],
        )
    } else if bounding.contains(&inner.start) & bounding.contains(&inner.end) {
        (Some(inner.clone()), vec![])
    } else if bounding.contains(&inner.start) {
        (
            Some(Range {
                start: inner.start,
                end: bounding.end,
            }),
            vec![Range {
                start: bounding.end,
                end: inner.end,
            }],
        )
    } else if bounding.contains(&inner.end) {
        (
            Some(Range {
                start: bounding.start,
                end: inner.end,
            }),
            vec![Range {
                start: inner.start,
                end: bounding.start,
            }],
        )
    } else {
        (None, vec![inner.clone()])
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Mapping {
    ranges: Vec<(Range<usize>, Range<usize>)>,
}

impl FromStr for Mapping {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .skip(1)
            .map(|l| {
                let nums: Vec<usize> = l
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                (
                    Range {
                        start: nums[1],
                        end: nums[1] + nums[2],
                    },
                    Range {
                        start: nums[0],
                        end: nums[0] + nums[2],
                    },
                )
            })
            .collect();

        Ok(Self { ranges })
    }
}

impl Mapping {
    fn map(&self, num: usize) -> usize {
        for (from, to) in &self.ranges {
            if from.contains(&num) {
                return to.start + num - from.start;
            }
        }

        num
    }

    fn map_ranges(&self, mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut mapped = Vec::new();

        let mut new_unmapped;
        for (from, to) in &self.ranges {
            new_unmapped = Vec::new();
            for um in ranges {
                let (inner, out) = containment(&um, &from);

                new_unmapped.extend(out);

                if let Some(inner) = inner {
                    mapped.push(Range {
                        start: to.start + inner.start - from.start,
                        end: to.start + inner.end - from.start,
                    });
                }
            }
            ranges = new_unmapped;
        }

        mapped.extend(ranges);
        mapped
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    seeds: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl FromStr for Game {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split("\n\n");

        let seeds = segments
            .next()
            .ok_or("Input not large enough")?
            .strip_prefix("seeds: ")
            .ok_or("Does not start with seed")?
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mappings = segments.map(Mapping::from_str).collect::<Result<_, _>>()?;

        Ok(Game { seeds, mappings })
    }
}

fn part_one(game: &Game) -> usize {
    game.seeds
        .iter()
        .map(|s| *&game.mappings.iter().fold(*s, |acc, map| map.map(acc)))
        .min()
        .unwrap()
}

fn part_two(game: &Game) -> usize {
    game.seeds
        .chunks_exact(2)
        .map(|v| {
            game.mappings.iter().fold(
                vec![Range {
                    start: v[0],
                    end: v[0] + v[1],
                }],
                |acc, map| map.map_ranges(acc),
            )
        })
        .flatten()
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = Game::from_str(&get_input("2023", "5"))?;

    let now = time::Instant::now();
    let sol_p1 = part_one(&input);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let now = time::Instant::now();
    let sol_p2 = part_two(&input);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse() {
        let game = Game::from_str(TEST).unwrap();
        assert_eq!(game.seeds, vec![79, 14, 55, 13]);
        assert_eq!(game.mappings[6].ranges[0].1, Range { start: 60, end: 97 });
    }

    #[test]
    fn test_part_one() {
        let game = Game::from_str(TEST).unwrap();
        assert_eq!(part_one(&game), 35);
    }

    #[test]
    fn test_part_two() {
        let game = Game::from_str(TEST).unwrap();
        assert_eq!(part_two(&game), 46);
    }
}

// use rustvent2023::get_input;
use std::time;

fn part_one() -> usize {
    42
}

fn part_two() -> usize {
    69
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let input = get_input("2022", "15");

    let now = time::Instant::now();
    let sol_p1 = part_one();
    println!(
        "Solution part one: {sol_p1} took: {}s",
        now.elapsed().as_secs_f32()
    );


    let now = time::Instant::now();
    let sol_p2 = part_two();
    println!(
        "Solution part two: {sol_p2} took: {}s",
        now.elapsed().as_secs_f32()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1701";

    #[test]
    fn test_parse() {
        let input = TEST;

        assert_eq!(input.parse::<isize>().unwrap(), 1701);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 42);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 69);
    }
}

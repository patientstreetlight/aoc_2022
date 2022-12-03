use aoc_2022::*;
use std::collections::HashMap;
use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-01.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    // println!("part2: {}", part2(&input));
}

type Input = ();

fn parse_input(s: &str) -> Input {
    todo!();
}

fn part1(input: &Input) -> i32 {
    todo!();
}

fn part2(input: &Input) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    //#[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 24000);
    }

    //#[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 45000);
    }

    //#[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 72511);
    }

    //#[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 212117);
    }
}

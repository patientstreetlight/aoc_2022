use aoc_2022::*;

const MY_INPUT: &str = include_str!("../../inputs/day-02.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<(u8, u8)>;

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let chars = line.as_bytes();
            (chars[0], chars[2])
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Outcome {
    Win,
    Tie,
    Loss,
}

impl From<u8> for Outcome {
    fn from(b: u8) -> Self {
        match b {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Tie,
            b'Z' => Outcome::Win,
            _ => panic!("bad input"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for RPS {
    fn from(b: u8) -> Self {
        match b {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("invalid input"),
        }
    }
}

fn play(opp: RPS, you: RPS) -> Outcome {
    match (you, opp) {
        (a, b) if a == b => Outcome::Tie,
        (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
            Outcome::Win
        }
        _ => Outcome::Loss,
    }
}

fn your_score(opp: RPS, you: RPS) -> i32 {
    base_points(you) + play_points(play(opp, you))
}

fn base_points(play: RPS) -> i32 {
    match play {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn play_points(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Loss => 0,
    }
}

fn part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|&(opp, you)| {
            let opp = RPS::from(opp);
            let you = RPS::from(you);
            your_score(opp, you)
        })
        .sum()
}

fn your_score2(opp: RPS, outcome: Outcome) -> i32 {
    let your_play = match outcome {
        Outcome::Tie => opp,
        Outcome::Loss => match opp {
            RPS::Paper => RPS::Rock,
            RPS::Rock => RPS::Scissors,
            RPS::Scissors => RPS::Paper,
        },
        Outcome::Win => match opp {
            RPS::Paper => RPS::Scissors,
            RPS::Rock => RPS::Paper,
            RPS::Scissors => RPS::Rock,
        },
    };
    base_points(your_play) + play_points(outcome)
}

fn part2(input: &Input) -> i32 {
    input
        .iter()
        .map(|&(opp, outcome)| {
            let opp = RPS::from(opp);
            let outcome = Outcome::from(outcome);
            your_score2(opp, outcome)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 12);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 15632);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 14416);
    }
}

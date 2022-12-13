use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-14.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Vec<(u16, u16)>>;

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            line.split(" -> ")
                .map(|xy| {
                    let (x, y) = xy.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn build_walls(input: &Input) -> HashSet<(u16, u16)> {
    let mut walls = HashSet::new();
    for points in input {
        let (mut prev_x, mut prev_y) = points[0];
        for &(x, y) in &points[1..] {
            if x == prev_x {
                let src = std::cmp::min(y, prev_y);
                let dst = std::cmp::max(y, prev_y);
                for y in src..=dst {
                    walls.insert((x, y));
                }
            } else if y == prev_y {
                let src = std::cmp::min(x, prev_x);
                let dst = std::cmp::max(x, prev_x);
                for x in src..=dst {
                    walls.insert((x, y));
                }
            } else {
                panic!("unexpected input");
            }
            prev_x = x;
            prev_y = y;
        }
    }
    walls
}

fn part1(input: &Input) -> i32 {
    let mut filled = build_walls(input);
    let floor = input
        .iter()
        .flat_map(|wall| wall.iter().map(|xy| xy.1))
        .max()
        .unwrap();
    for count in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y >= floor {
                return count;
            }
            if !filled.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !filled.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !filled.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            filled.insert((x, y));
            break;
        }
    }
    panic!("unreachable")
}

fn part2(input: &Input) -> i32 {
    let mut filled = build_walls(input);
    let floor = input
        .iter()
        .flat_map(|wall| wall.iter().map(|xy| xy.1))
        .max()
        .unwrap()
        + 2;
    for count in 1.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y + 1 == floor {
                filled.insert((x, y));
                break;
            }
            if !filled.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !filled.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !filled.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            filled.insert((x, y));
            if x == 500 && y == 0 {
                return count;
            }
            break;
        }
    }
    panic!("unreachable")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 93);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 808);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 26625);
    }
}

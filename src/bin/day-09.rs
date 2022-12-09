use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

const MY_INPUT: &str = include_str!("../../inputs/day-09.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<(Dir, u8)>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dir {
    U, D, L, R,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            "L" => Ok(Dir::L),
            "R" => Ok(Dir::R),
            _ => Err(()),
        }
    }
}

fn parse_input(s: &str) -> Input {
    s.lines().map(|line| {
        let instruction: Vec<_> = line.split_ascii_whitespace().collect();
        (instruction[0].parse::<Dir>().unwrap(), instruction[1].parse::<u8>().unwrap())
    })
    .collect()
}

fn part1(input: &Input) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // coordinates are (x, y), with positive y being "up" and
    // positive x being "right".
    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);
    visited.insert(tail_pos);
    for &(dir, count) in input {
        for _ in 0..count {
            match dir {
                Dir::U => head_pos.1 += 1,
                Dir::D => head_pos.1 -= 1,
                Dir::L => head_pos.0 -= 1,
                Dir::R => head_pos.0 += 1,
            }
            if mv_tail(head_pos, &mut tail_pos) {
                visited.insert(tail_pos);
            }
        }
    }
    visited.len()
}

fn mv_tail(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
    if (head.0 - tail.0).abs() > 1 {
        // head is either too far left or right of tail, but should be
        // within 1 vertical unit of tail.
        match tail.1.cmp(&head.1) {
            Ordering::Less => tail.1 += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.1 -= 1,
        }
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
        true
    } else if (head.1 - tail.1).abs() > 1 {
        // head is either too far above or below tail, but should
        // be within 1 horizontal unit of tail.
        match tail.0.cmp(&head.0) {
            Ordering::Less => tail.0 += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.0 -= 1,
        }
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
        true
    } else {
        false
    }
}

fn part2(input: &Input) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut rope = [(0, 0); 10];
    for &(dir, count) in input {
        for _ in 0..count {
            match dir {
                Dir::U => rope[0].1 += 1,
                Dir::D => rope[0].1 -= 1,
                Dir::L => rope[0].0 -= 1,
                Dir::R => rope[0].0 += 1,
            }
            for i in 0..(rope.len() - 1) {
                let head = rope[i];
                let tail = &mut rope[i+1];
                if !mv_tail(head, tail) {
                    break;
                }
                if i == rope.len() - 2 {
                    visited.insert(rope.last().copied().unwrap());
                }
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_SAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(LARGER_SAMPLE);
        assert_eq!(part2(&input), 36);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 6266);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 2369);
    }
}

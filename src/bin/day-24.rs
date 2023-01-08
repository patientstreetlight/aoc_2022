use aoc_2022::*;
use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-24.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

type Input = Grid<Option<Dir>>;

fn parse_input(s: &str) -> Input {
    let rows: Vec<Vec<Option<Dir>>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => Some(Dir::L),
                    '>' => Some(Dir::R),
                    'v' => Some(Dir::D),
                    '^' => Some(Dir::U),
                    _ => None,
                })
                .collect()
        })
        .collect();
    rows.into()
}

fn blizzard_positions(input: &Input, turn: usize) -> Vec<Vec<bool>> {
    let mut ps = vec![vec![false; input.num_cols()]; input.num_rows()];
    for ((r, c), dir) in input.enumerated_elems() {
        let (future_r, future_c) = match dir {
            None => continue,
            Some(Dir::U) => {
                let turn = turn % (input.num_rows() - 2);
                // going up 1 is like going down (input.num_rows() - 2 - 1)
                // going up 2 is like going down (input.num_rows() - 2 - 2)
                let down = input.num_rows() - 2 - turn;
                let new_r = (r - 1 + down) % (input.num_rows() - 2) + 1;
                (new_r, c)
            }
            Some(Dir::D) => ((r - 1 + turn) % (input.num_rows() - 2) + 1, c),
            Some(Dir::L) => {
                let turn = turn % (input.num_cols() - 2);
                let right = input.num_cols() - 2 - turn;
                let new_c = (c - 1 + right) % (input.num_cols() - 2) + 1;
                (r, new_c)
            }
            Some(Dir::R) => (r, (c - 1 + turn) % (input.num_cols() - 2) + 1),
        };
        ps[future_r][future_c] = true;
    }
    ps
}

const DELTAS: &[(i32, i32)] = &[(0, 0), (-1, 0), (1, 0), (0, 1), (0, -1)];

fn part1(input: &Input) -> usize {
    let start = (0, 1);
    let goal = ((input.num_rows() - 1) as i32, (input.num_cols() - 2) as i32);
    reach_goal(input, 0, start, goal)
}

fn reach_goal(input: &Input, start_time: usize, start: (i32, i32), goal: (i32, i32)) -> usize {
    let mut positions = vec![start];
    let end = ((input.num_rows() - 1) as i32, (input.num_cols() - 2) as i32);
    for t in (start_time + 1).. {
        let mut next_positions = HashSet::new();
        let blizzards = blizzard_positions(input, t);
        for (r, c) in positions {
            let deltas = if (r, c) == (0, 1) {
                &[(0, 0), (1, 0)]
            } else if (r, c) == end {
                &[(0, 0), (-1, 0)]
            } else {
                DELTAS
            };
            for &(dr, dc) in deltas {
                let n @ (nr, nc) = (r + dr, c + dc);
                if n == goal {
                    return t;
                }
                if (nr == 0 && (nr, nc) != start)
                    || (nr as usize == input.num_rows() - 1 && (nr, nc) != start)
                    || nc == 0
                    || nc as usize == input.num_cols() - 1
                    || blizzards[nr as usize][nc as usize]
                {
                    continue;
                }
                next_positions.insert(n);
            }
        }
        positions = next_positions.into_iter().collect();
    }
    panic!("unreachable");
}

fn part2(input: &Input) -> usize {
    let start = (0, 1);
    let goal = ((input.num_rows() - 1) as i32, (input.num_cols() - 2) as i32);
    let a = reach_goal(input, 0, start, goal);
    let b = reach_goal(input, a, goal, start);
    reach_goal(input, b, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 247);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 728);
    }
}

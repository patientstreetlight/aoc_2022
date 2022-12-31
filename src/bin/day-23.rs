use std::collections::HashMap;
use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-23.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = HashSet<(i32, i32)>;

fn parse_input(s: &str) -> Input {
    let mut positions = HashSet::new();
    for (r, line) in s.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                positions.insert((r as i32, c as i32));
            }
        }
    }
    positions
}

const DELTAS: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn neighbors((r, c): (i32, i32)) -> impl Iterator<Item=(i32, i32)> {
    DELTAS.iter().map(move |&(dr, dc)| (r + dr, c + dc))
}

const NORTH_DELTAS: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1)];

fn north_neighbors((r, c): (i32, i32)) -> Box<dyn Iterator<Item=(i32, i32)>> {
    Box::new(NORTH_DELTAS.iter().map(move |&(dr, dc)| (r + dr, c + dc)))
}

const SOUTH_DELTAS: &[(i32, i32)] = &[(1, -1), (1, 0), (1, 1)];

fn south_neighbors((r, c): (i32, i32)) -> Box<dyn Iterator<Item=(i32, i32)>> {
    Box::new(SOUTH_DELTAS.iter().map(move |&(dr, dc)| (r + dr, c + dc)))
}

const EAST_DELTAS: &[(i32, i32)] = &[(-1, 1), (0, 1), (1, 1)];

fn east_neighbors((r, c): (i32, i32)) -> Box<dyn Iterator<Item=(i32, i32)>> {
    Box::new(EAST_DELTAS.iter().map(move |&(dr, dc)| (r + dr, c + dc)))
}

const WEST_DELTAS: &[(i32, i32)] = &[(-1, -1), (0, -1), (1, -1)];

fn west_neighbors((r, c): (i32, i32)) -> Box<dyn Iterator<Item=(i32, i32)>> {
    Box::new(WEST_DELTAS.iter().map(move |&(dr, dc)| (r + dr, c + dc)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

const DIRS: &[Dir] = &[Dir::N, Dir::S, Dir::W, Dir::E];

fn part1(input: &Input) -> i32 {
    let mut positions = input.clone();
    for round in 0..10 {
        let mut next_positions = HashSet::new();
        // Maps proposed spot back to the elves who want to move there.
        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        let is_free = |elf| !positions.contains(&elf);
        for &elf@(r, c) in &positions {
            if neighbors(elf).all(is_free) {
                // stays
                next_positions.insert(elf);
            } else {
                // proposes to move
                let mut proposed = false;
                for dir in DIRS.iter().cycle().skip(round % 4).take(4) {
                    let (pos, mut neighbors) = match dir {
                        Dir::N => ((r-1, c), north_neighbors(elf)),
                        Dir::S => ((r+1, c), south_neighbors(elf)),
                        Dir::W => ((r, c-1), west_neighbors(elf)),
                        Dir::E => ((r, c+1), east_neighbors(elf)),
                    };
                    if neighbors.all(is_free) {
                        proposals.entry(pos).or_default().push(elf);
                        proposed = true;
                        break;
                    }
                }
                if !proposed {
                    proposals.entry(elf).or_default().push(elf);
                }
            }
        }
        // resolve proposals
        for (new_pos, elves) in &proposals {
            if elves.len() == 1 {
                next_positions.insert(*new_pos);
            } else {
                for elf in elves {
                    next_positions.insert(*elf);
                }
            }
        }
        assert_eq!(positions.len(), next_positions.len());
        positions = next_positions;
    }
    let min_row = positions.iter().copied().map(|(r, _)| r).min().unwrap();
    let max_row = positions.iter().copied().map(|(r, _)| r).max().unwrap();
    let min_col = positions.iter().copied().map(|(_, c)| c).min().unwrap();
    let max_col = positions.iter().copied().map(|(_, c)| c).max().unwrap();
    (max_row - min_row + 1) * (max_col - min_col + 1) - positions.len() as i32
}

fn part2(input: &Input) -> usize {
    let mut positions = input.clone();
    for round in 0.. {
        let mut next_positions = HashSet::new();
        // Maps proposed spot back to the elves who want to move there.
        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        let is_free = |elf| !positions.contains(&elf);
        let mut stable = true;
        for &elf@(r, c) in &positions {
            if neighbors(elf).all(is_free) {
                // stays
                next_positions.insert(elf);
            } else {
                // proposes to move
                stable = false;
                let mut proposed = false;
                for dir in DIRS.iter().cycle().skip(round % 4).take(4) {
                    let (pos, mut neighbors) = match dir {
                        Dir::N => ((r-1, c), north_neighbors(elf)),
                        Dir::S => ((r+1, c), south_neighbors(elf)),
                        Dir::W => ((r, c-1), west_neighbors(elf)),
                        Dir::E => ((r, c+1), east_neighbors(elf)),
                    };
                    if neighbors.all(is_free) {
                        proposals.entry(pos).or_default().push(elf);
                        proposed = true;
                        break;
                    }
                }
                if !proposed {
                    proposals.entry(elf).or_default().push(elf);
                }
            }
        }
        if stable {
            return round + 1;
        }
        // resolve proposals
        for (new_pos, elves) in &proposals {
            if elves.len() == 1 {
                next_positions.insert(*new_pos);
            } else {
                for elf in elves {
                    next_positions.insert(*elf);
                }
            }
        }
        assert_eq!(positions.len(), next_positions.len());
        positions = next_positions;
    }
    panic!("unreachable");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 20);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 3970);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 923);
    }
}

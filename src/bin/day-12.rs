use std::collections::HashSet;

use aoc_2022::Grid;

const MY_INPUT: &str = include_str!("../../inputs/day-12.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

struct Input {
    heights: Grid<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(s: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights = Grid::from_str(s, |rc, cs| {
        cs.next().map(|c| match c {
            'S' => {
                start = rc;
                0
            }
            'E' => {
                end = rc;
                b'z' - b'a'
            }
            _ => c as u8 - b'a',
        })
    });
    Input {
        heights,
        start,
        end,
    }
}

fn part1(input: &Input) -> i32 {
    let mut frontier = vec![input.start];
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();
    discovered.insert(input.start);
    let mut distance = 1;
    while !frontier.is_empty() {
        let mut next_frontier = vec![];
        for rc in frontier {
            let height = input.heights[rc];
            for n in input.heights.neighbors4(rc) {
                let n_height = input.heights[n];
                if n_height > height + 1 {
                    continue;
                }
                if n == input.end {
                    return distance;
                }
                if discovered.insert(n) {
                    next_frontier.push(n);
                }
            }
        }
        distance += 1;
        frontier = next_frontier;
    }
    panic!("No path found");
}

fn part2(input: &Input) -> u16 {
    let mut frontier = vec![input.end];
    let mut discovered = HashSet::new();
    discovered.insert(input.end);
    let mut distance = 1;
    while !frontier.is_empty() {
        let mut new_frontier = vec![];
        for rc in frontier {
            let height = input.heights[rc];
            for n in input.heights.neighbors4(rc) {
                let n_height = input.heights[n];
                if height > n_height + 1 {
                    continue;
                }
                if n_height == 0 {
                    return distance;
                }
                if discovered.insert(n) {
                    new_frontier.push(n);
                }
            }
        }
        frontier = new_frontier;
        distance += 1;
    }
    panic!("No path found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 29);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 361);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 354);
    }
}

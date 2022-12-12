use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-12.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

struct Input {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Input {
    fn neighbors(&self, (r, c): (usize, usize)) -> Vec<(usize, usize)> {
        let r = r as i32;
        let c = c as i32;
        let rows = self.heights.len() as i32;
        let cols = self.heights[0].len() as i32;
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|&(r, c)| 0 <= r && r < rows && 0 <= c && c < cols)
            .map(|(r, c)| (r as usize, c as usize))
            .collect()
    }
}

fn parse_input(s: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights = s
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, height)| match height {
                    'S' => {
                        start = (r, c);
                        0
                    }
                    'E' => {
                        end = (r, c);
                        b'z' - b'a'
                    }
                    h => h as u8 - b'a',
                })
                .collect()
        })
        .collect();
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
        for rc @ (r, c) in frontier {
            let height = input.heights[r][c];
            for n @ (nr, nc) in input.neighbors(rc) {
                let n_height = input.heights[nr][nc];
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
        for rc @ (r, c) in frontier {
            let height = input.heights[r][c];
            for n @ (nr, nc) in input.neighbors(rc) {
                let n_height = input.heights[nr][nc];
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

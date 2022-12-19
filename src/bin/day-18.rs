use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-18.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<(i8, i8, i8)>;

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let mut xyz = line.split(',').map(|n| n.parse().unwrap());
            let x = xyz.next().unwrap();
            let y = xyz.next().unwrap();
            let z = xyz.next().unwrap();
            (x, y, z)
        })
        .collect()
}

const NEIGHBOR_DELTAS: &[(i8, i8, i8)] = &[
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn part1(input: &Input) -> i32 {
    let mut surface_area = 0;
    let mut grid: HashSet<(i8, i8, i8)> = HashSet::new();
    for (x, y, z) in input.iter().copied() {
        let mut num_neighbors = 0;
        for &(dx, dy, dz) in NEIGHBOR_DELTAS {
            if grid.contains(&(x + dx, y + dy, z + dz)) {
                num_neighbors += 1;
            }
        }
        surface_area += 6 - 2 * num_neighbors;
        grid.insert((x, y, z));
    }
    surface_area
}

fn part2(input: &Input) -> i32 {
    let min_x = input.iter().map(|xyz| xyz.0).min().unwrap() - 1;
    let max_x = input.iter().map(|xyz| xyz.0).max().unwrap() + 1;
    let min_y = input.iter().map(|xyz| xyz.1).min().unwrap() - 1;
    let max_y = input.iter().map(|xyz| xyz.1).max().unwrap() + 1;
    let min_z = input.iter().map(|xyz| xyz.2).min().unwrap() - 1;
    let max_z = input.iter().map(|xyz| xyz.2).max().unwrap() + 1;

    let lava: HashSet<_> = input.iter().copied().collect();

    let mut sa = 0;
    let air_corner = (min_x, min_y, min_z);
    let mut frontier = vec![air_corner];
    let mut discovered: HashSet<(i8, i8, i8)> = HashSet::new();
    discovered.insert(air_corner);
    while let Some(n) = frontier.pop() {
        for &(dx, dy, dz) in NEIGHBOR_DELTAS {
            let x = n.0 + dx;
            let y = n.1 + dy;
            let z = n.2 + dz;
            if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
                continue;
            }
            let xyz = (x, y, z);
            if lava.contains(&xyz) {
                sa += 1;
            } else if discovered.insert(xyz) {
                frontier.push(xyz);
            }
        }
    }
    sa
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 58);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 4636);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 2572);
    }
}

const MY_INPUT: &str = include_str!("../../inputs/day-04.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<((u8, u8), (u8, u8))>;

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let (r1, r2) = line.split_once(',').unwrap();
            let (a, b) = r1.split_once('-').unwrap();
            let (c, d) = r2.split_once('-').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            let c = c.parse().unwrap();
            let d = d.parse().unwrap();
            ((a, b), (c, d))
        })
        .collect()
}

fn contains(range1: (u8, u8), range2: (u8, u8)) -> bool {
    let (a, b) = range1;
    let (c, d) = range2;
    (a <= c && d <= b) || (c <= a && b <= d)
}

fn overlaps(range1: (u8, u8), range2: (u8, u8)) -> bool {
    let (a, b) = range1;
    let (c, d) = range2;
    contains(range1, range2) || (a <= d && d <= b) || (a <= c && c <= b)
}

fn part1(input: &Input) -> i32 {
    input.iter().filter(|&&(a, b)| contains(a, b)).count() as i32
}

fn part2(input: &Input) -> i32 {
    input.iter().filter(|&&(a, b)| overlaps(a, b)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 556);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 876);
    }
}

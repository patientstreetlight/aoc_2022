const MY_INPUT: &str = include_str!("../../inputs/day-03.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<&'static [u8]>;

fn parse_input(s: &'static str) -> Input {
    s.lines().map(|line| line.as_bytes()).collect()
}

fn priority(c: u8) -> i32 {
    match c {
        b'a'..=b'z' => (c - b'a' + 1) as i32,
        b'A'..=b'Z' => (c - b'A' + 27) as i32,
        _ => panic!(""),
    }
}

fn common_elem(rucksack: &[u8]) -> u8 {
    let mid = rucksack.len() / 2;
    let comp1 = &rucksack[0..mid];
    let comp2 = &rucksack[mid..];
    let in_comp1 = char_set(comp1);
    let in_comp2 = char_set(comp2);
    let intersection = in_comp1 & in_comp2;
    find_elem(intersection)
}

fn char_set(cs: &[u8]) -> u64 {
    let mut s = 0;
    for c in cs {
        let p = priority(*c);
        s |= 1 << p;
    }
    s
}

fn find_elem(set: u64) -> u8 {
    for i in 0..64 {
        if (set >> i) & 1 == 1 {
            return match i {
                1..=26 => b'a' + i - 1,
                _ => b'A' + i - 27,
            };
        }
    }
    panic!("empty set");
}

fn part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|&rucksack| priority(common_elem(rucksack)))
        .sum()
}

fn get_badge(rucksacks: &[&[u8]]) -> u8 {
    let intersection = rucksacks
        .iter()
        .map(|rs| char_set(rs))
        .reduce(|s1, s2| s1 & s2)
        .unwrap();
    find_elem(intersection)
}

fn part2(input: &Input) -> i32 {
    input
        .chunks_exact(3)
        .map(|chunk| priority(get_badge(chunk)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 70);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 7795);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 2703);
    }
}

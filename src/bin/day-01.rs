use aoc_2022::*;

const MY_INPUT: &str = include_str!("../../inputs/day-01.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Vec<i32>>;

fn parse_input(s: &str) -> Input {
    grouped_lines(s)
        .map(|group|
            group.into_iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect())
        .collect()
}

fn part2(elves: &Input) -> i32 {
    // sorted lowest to highest
    let mut top_3 = [0, 0, 0];
    for elf in elves {
        let calories = elf.iter().sum();
        if calories > top_3[0] {
            top_3[0] = calories;
            let mut i = 0;
            while i < top_3.len() - 1 && top_3[i] > top_3[i+1] {
                top_3.swap(i, i+1);
                i += 1;
            }
        }
    }
    top_3.into_iter().sum()
}

fn part1(elves: &Input) -> i32 {
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 45000);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 72511);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 212117);
    }
}
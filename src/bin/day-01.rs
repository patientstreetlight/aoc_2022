use aoc_2022::*;

fn main() {
    let input = parse_input(include_str!("../../inputs/day-01.txt"));
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
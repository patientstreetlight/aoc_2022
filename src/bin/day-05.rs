const MY_INPUT: &str = include_str!("../../inputs/day-05.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(u8, u8, u8)>,
}

fn parse_input(s: &str) -> Input {
    let (stacks, moves) = s.split_once("\n\n").unwrap();
    let mut real_stacks = vec![];
    for line in stacks.lines().rev().skip(1) {
        let line = line.as_bytes();
        for (i, cell) in line.chunks(4).enumerate() {
            while i >= real_stacks.len() {
                real_stacks.push(vec![]);
            }
            if cell[0] == b'[' {
                real_stacks[i].push(cell[1] as char);
            }
        }
    }
    let instructions = moves
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();
            let count: u8 = words[1].parse().unwrap();
            let src: u8 = words[3].parse().unwrap();
            let dest: u8 = words[5].parse().unwrap();
            (count, src - 1, dest - 1)
        })
        .collect();

    Input {
        stacks: real_stacks,
        instructions,
    }
}

fn stack_tops(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for &(count, src, dest) in &input.instructions {
        for _ in 0..count {
            let val = stacks[src as usize].pop().unwrap();
            stacks[dest as usize].push(val);
        }
    }
    stack_tops(&stacks)
}

fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for &(count, src, dest) in &input.instructions {
        let src = &mut stacks[src as usize];
        let i = src.len() - count as usize;
        let elems: Vec<_> = src.drain(i..).collect();
        stacks[dest as usize].extend_from_slice(&elems);
    }
    stack_tops(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), "MCD");
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), "LJSVLTWQM");
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), "BRQWDBBJM");
    }
}

use itertools::Itertools;

const MY_INPUT: &str = include_str!("../../inputs/day-11.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Monkey>;

type Item = u64;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Val {
    Const(Item),
    Old,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Op {
    Plus(Val),
    Times(Val),
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    op: Op,
    test_divisible: Item,
    true_branch: usize,
    false_branch: usize,
}

fn parse_input(s: &str) -> Input {
    let monkeys = s.split("\n\n");
    monkeys
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next(); // Monkey: line
            let items: Vec<Item> = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(2)
                .map(|i| i.trim_end_matches(',').parse().unwrap())
                .collect();
            let op: Vec<_> = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(4)
                .collect();
            let val = match op[1] {
                "old" => Val::Old,
                val => Val::Const(val.parse().unwrap()),
            };
            let op = match op[0] {
                "+" => Op::Plus(val),
                "*" => Op::Times(val),
                _ => panic!("bad operator"),
            };
            let test_divisible: Item = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .nth(3)
                .unwrap()
                .parse()
                .unwrap();
            let true_branch: usize = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .nth(5)
                .unwrap()
                .parse()
                .unwrap();
            let false_branch: usize = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .nth(5)
                .unwrap()
                .parse()
                .unwrap();
            Monkey {
                items,
                op,
                test_divisible,
                true_branch,
                false_branch,
            }
        })
        .collect()
}

fn apply_op(op: Op, worry: Item) -> Item {
    match op {
        Op::Plus(Val::Const(c)) => worry + c,
        Op::Plus(Val::Old) => worry * 2,
        Op::Times(Val::Const(c)) => worry * c,
        Op::Times(Val::Old) => worry * worry,
    }
}

fn part1(monkeys: &Input) -> usize {
    let mut entries_per_monkey: Vec<_> =
        monkeys.iter().map(|monkey| monkey.items.clone()).collect();
    let mut inspection_counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let items = std::mem::take(&mut entries_per_monkey[i]);
            inspection_counts[i] += items.len();
            for item in items {
                let item = apply_op(monkey.op, item) / 3;
                let next_monkey = if item % monkey.test_divisible == 0 {
                    monkey.true_branch
                } else {
                    monkey.false_branch
                };
                entries_per_monkey[next_monkey].push(item);
            }
        }
    }
    inspection_counts.sort();
    let n = inspection_counts.len();
    inspection_counts[n - 1] * inspection_counts[n - 2]
}

fn part2(monkeys: &Input) -> usize {
    let mut entries_per_monkey: Vec<_> =
        monkeys.iter().map(|monkey| monkey.items.clone()).collect();
    let base: Item = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible)
        .product1()
        .unwrap();
    let mut inspection_counts = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let items = std::mem::take(&mut entries_per_monkey[i]);
            inspection_counts[i] += items.len();
            for item in items {
                let item = apply_op(monkey.op, item) % base;
                let next_monkey = if item % monkey.test_divisible == 0 {
                    monkey.true_branch
                } else {
                    monkey.false_branch
                };
                entries_per_monkey[next_monkey].push(item);
            }
        }
    }
    inspection_counts.sort();
    let n = inspection_counts.len();
    inspection_counts[n - 1] * inspection_counts[n - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 2713310158);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 182293);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 54832778815);
    }
}

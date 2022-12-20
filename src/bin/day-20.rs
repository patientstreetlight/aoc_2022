use std::cmp::Ordering;

const MY_INPUT: &str = include_str!("../../inputs/day-20.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<i64>;

fn parse_input(s: &str) -> Input {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

/// returns a pair, (prevs, nexts), such that for all i, 0 <= i < input.len():
/// * nexts[prevs[i]] = i
/// * prevs[nexts[i]] = i
/// The prevs and nexts indices form a kind of doubly linked list of the elements
/// of the input.
fn mk_list(input: &Input) -> (Vec<usize>, Vec<usize>) {
    let n = input.len();
    let mut prevs = Vec::with_capacity(input.len());
    let mut nexts = Vec::with_capacity(input.len());
    for i in 0..n {
        let prev = if i == 0 { n - 1 } else { i - 1 };
        let next = (i + 1) % n;
        prevs.push(prev);
        nexts.push(next);
    }
    (prevs, nexts)
}

fn mix(input: &Input, num_iters: usize) -> i64 {
    let (mut prevs, mut nexts) = mk_list(input);
    let n = input.len();
    for _ in 0..num_iters {
        for (i, val) in input.iter().enumerate() {
            match val.cmp(&0) {
                Ordering::Less => {
                    // splice out this number.
                    let prev = prevs[i];
                    let next = nexts[i];
                    nexts[prev] = next;
                    prevs[next] = prev;
                    // splice it in at the correct location
                    let mut parent = next;
                    let val = (-*val as usize) % (n - 1);
                    for _ in 0..val {
                        parent = prevs[parent];
                    }
                    // parent is now the new successor
                    let parent_prev = prevs[parent];
                    prevs[parent] = i;
                    nexts[i] = parent;
                    nexts[parent_prev] = i;
                    prevs[i] = parent_prev;
                }
                Ordering::Equal => {
                    // nothing
                }
                Ordering::Greater => {
                    // move forward
                    // splice out this number.
                    let prev = prevs[i];
                    let next = nexts[i];
                    nexts[prev] = next;
                    prevs[next] = prev;
                    // splice it in at the correct location
                    let val: usize = *val as usize % (n - 1);
                    let mut parent = prev;
                    for _ in 0..val {
                        parent = nexts[parent];
                    }
                    // parent is now the new prededessor
                    let parent_next = nexts[parent];
                    nexts[parent] = i;
                    prevs[i] = parent;
                    prevs[parent_next] = i;
                    nexts[i] = parent_next;
                }
            }
        }
    }
    grove_coordinates(input, &nexts)
}

fn grove_coordinates(input: &Input, nexts: &[usize]) -> i64 {
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| num_at(input, nexts, i))
        .sum()
}

fn part1(input: &Input) -> i64 {
    mix(input, 1)
}

fn num_at(vals: &[i64], nexts: &[usize], i: usize) -> i64 {
    let mut idx = vals.iter().copied().position(|e| e == 0).unwrap();
    let i = i % vals.len();
    for _ in 0..i {
        idx = nexts[idx];
    }
    vals[idx]
}

const DECRYPTION_KEY: i64 = 811589153;

fn part2(input: &Input) -> i64 {
    let input: Vec<_> = input.iter().copied().map(|n| n * DECRYPTION_KEY).collect();
    mix(&input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 1623178306);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 7225);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 548634267428);
    }
}

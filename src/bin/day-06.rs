const MY_INPUT: &str = include_str!("../../inputs/day-06.txt");

fn main() {
    println!("part1: {}", part1(MY_INPUT));
    println!("part2: {}", part2(MY_INPUT));
}

fn search(s: &str, len: usize) -> usize {
    let cs = s.as_bytes();
    cs.windows(len)
        .enumerate()
        .find(|&(_, w)| {
            let mut set: u32 = 0;
            w.iter().all(|c| {
                let c = 1 << (c - b'a');
                if set & c != 0 {
                    false
                } else {
                    set |= c;
                    true
                }
            })
        })
        .map(|(i, _)| i + len)
        .unwrap()
}

fn part1(input: &str) -> usize {
    search(input, 4)
}

fn part2(input: &str) -> usize {
    search(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn part1_my_input() {
        assert_eq!(part1(MY_INPUT), 1912);
    }

    #[test]
    fn part2_my_input() {
        assert_eq!(part2(MY_INPUT), 2122);
    }
}

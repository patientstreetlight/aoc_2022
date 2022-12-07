const MY_INPUT: &str = include_str!("../../inputs/day-06.txt");

fn main() {
    println!("part1: {}", part1(MY_INPUT));
    println!("part2: {}", part2(MY_INPUT));
}

fn search(s: &str, len: usize) -> usize {
    fn to_index(c: u8) -> usize {
        (c - b'a') as usize
    }
    let cs = s.as_bytes();
    // - i <= j
    // - all characters in cs[i..j] are unique
    // - for all k, i <= k < j, last_index_of[to_index(cs[k])] = Some(k)
    let mut last_index_of: [Option<usize>; 26] = [None; 26];
    let mut i = 0;
    let mut j = 0;
    while j - i != len {
        let c = to_index(cs[j]);
        if let Some(k) = last_index_of[c] {
            while i <= k {
                last_index_of[to_index(cs[i])] = None;
                i += 1;
            }
        }
        last_index_of[c] = Some(j);
        j += 1;
    }
    j
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

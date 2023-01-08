const MY_INPUT: &str = include_str!("../../inputs/day-25.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
}

type Input = Vec<&'static str>;

fn parse_input(s: &'static str) -> Input {
    s.lines().collect()
}

fn snafu_to_dec(s: &str) -> i128 {
    let mut base = 1;
    let mut n = 0;
    for c in s.chars().rev() {
        let d = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("bad snafu number"),
        };
        n += base * d;
        base *= 5;
    }
    n
}

fn dec_to_snafu(n: i128) -> String {
    // bound[d] = max number representable with d snafu digits.
    let mut bound = [0; 25];
    let mut base = 1;
    for d in 1..bound.len() {
        bound[d] = 2 * base + bound[d - 1];
        base *= 5;
    }
    let mut snafu_digits_needed = 0;
    for (d, b) in bound.iter().copied().enumerate() {
        if b >= n {
            snafu_digits_needed = d;
        }
    }
    let mut base_5_equivalent = n;
    let mut base = 1;
    for _ in 0..snafu_digits_needed {
        base_5_equivalent += 2 * base;
        base *= 5;
    }
    let mut digits = vec![];
    while base_5_equivalent != 0 {
        let digit = base_5_equivalent % 5;
        base_5_equivalent /= 5;
        let digit = match digit {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("unreachable"),
        };
        digits.push(digit);
    }
    digits.into_iter().rev().skip_while(|c| *c == '0').collect()
}

fn part1(input: &Input) -> String {
    let s: i128 = input.iter().copied().map(snafu_to_dec).sum();
    dec_to_snafu(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), "2=-1=0");
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), "2=2-1-010==-0-1-=--2");
    }
}

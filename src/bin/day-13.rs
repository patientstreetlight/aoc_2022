use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

const MY_INPUT: &str = include_str!("../../inputs/day-13.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[derive(Debug, Clone)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().peekable();
        Ok(parse_packet(&mut cs))
    }
}

fn parse_packet(cs: &mut Peekable<Chars>) -> Packet {
    match cs.peek().unwrap() {
        '[' => {
            cs.next();
            Packet::List(parse_list(cs))
        }
        n if n.is_ascii_digit() => Packet::Num(parse_number(cs)),
        _ => panic!("malformed packet"),
    }
}

fn parse_number(cs: &mut Peekable<Chars>) -> u8 {
    let mut val = cs.next().unwrap().to_digit(10).unwrap();
    loop {
        match cs.peek() {
            Some(n) if n.is_ascii_digit() => {
                let n = cs.next().unwrap();
                val = val * 10 + n.to_digit(10).unwrap();
            }
            _ => return val as u8,
        }
    }
}

fn parse_list(cs: &mut Peekable<Chars>) -> Vec<Packet> {
    let mut list = vec![];
    if let Some(']') = cs.peek() {
        cs.next();
        return list;
    }
    loop {
        let packet = parse_packet(cs);
        list.push(packet);
        match cs.next().unwrap() {
            ',' => continue,
            ']' => return list,
            _ => panic!("malformed packet"),
        }
    }
}

type Input = Vec<(Packet, Packet)>;

fn parse_input(s: &str) -> Input {
    s.split("\n\n")
        .map(|pair| {
            let (p1, p2) = pair.split_once('\n').unwrap();
            let p1 = p1.parse().unwrap();
            let p2 = p2.parse().unwrap();
            (p1, p2)
        })
        .collect()
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        packet_order(self, other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(packet_order(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        packet_order(self, other)
    }
}

fn packet_order(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
        (Packet::List(l), Packet::List(r)) => {
            let mut l = &l[..];
            let mut r = &r[..];
            loop {
                match (l, r) {
                    ([], []) => return Ordering::Equal,
                    ([], _) => return Ordering::Less,
                    (_, []) => return Ordering::Greater,
                    ([l_head, l_rest @ ..], [r_head, r_rest @ ..]) => {
                        match packet_order(l_head, r_head) {
                            Ordering::Equal => {
                                l = l_rest;
                                r = r_rest;
                            }
                            non_eq => return non_eq,
                        }
                    }
                }
            }
        }
        (Packet::List(_), r @ Packet::Num(_)) => {
            let right = Packet::List(vec![r.clone()]);
            packet_order(left, &right)
        }
        (l @ Packet::Num(_), Packet::List(_)) => {
            let left = Packet::List(vec![l.clone()]);
            packet_order(&left, right)
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| if left < right { Some(i + 1) } else { None })
        .sum()
}

fn part2(input: &Input) -> usize {
    let mut all_packets = vec![];
    for (p1, p2) in input {
        all_packets.push(p1);
        all_packets.push(p2);
    }
    let dividers = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    for d in &dividers {
        all_packets.push(d);
    }
    all_packets.sort_unstable();
    all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| {
            if dividers.iter().any(|div| div == p) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 140);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 5684);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 22932);
    }
}

use std::collections::HashMap;

use aoc_2022::Grid;

const MY_INPUT: &str = include_str!("../../inputs/day-16.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

struct Input {
    start: u8,
    valves: Vec<Valve>,
}

struct Valve {
    flow_rate: u8,
    tunnels: Vec<u8>,
}

/// Returns a grid, g, where g[(src, dst)] is the min distance from node src to node dst.
fn min_distances(input: &[Valve]) -> Grid<u8> {
    let min_distances = vec![vec![None; input.len()]; input.len()];
    let mut min_distances: Grid<Option<u8>> = Grid::from(min_distances);
    for i in 0..input.len() {
        min_distances[(i, i)] = Some(0);
    }
    let mut pairs_of_distance_i_minus_one: Vec<(u8, u8)> =
        (0..input.len()).map(|i| (i as u8, i as u8)).collect();
    for i in 1..=input.len() {
        let mut pairs_of_distance_i = vec![];
        for (src, dst) in pairs_of_distance_i_minus_one {
            for &n in &input[dst as usize].tunnels {
                if min_distances[(src as usize, n as usize)].is_none() {
                    min_distances[(src as usize, n as usize)] = Some(i as u8);
                    pairs_of_distance_i.push((src, n));
                }
            }
        }
        pairs_of_distance_i_minus_one = pairs_of_distance_i;
    }
    min_distances.map(|md| md.unwrap())
}

fn parse_input(s: &str) -> Input {
    let parsed: HashMap<&str, (u8, Vec<&str>)> = s
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();
            let valve_name = words[1];
            let flow_rate = words[4]; // rate=0;
            let flow_rate = flow_rate
                .split_once('=')
                .unwrap()
                .1
                .trim_end_matches(';')
                .parse()
                .unwrap();
            let tunnels = &words[9..];
            let tunnels: Vec<_> = tunnels
                .iter()
                .map(|tunnel| tunnel.trim_end_matches(','))
                .collect();
            let valve = (flow_rate, tunnels);
            (valve_name, valve)
        })
        .collect();
    // normalize input to use integer ids rather than strings.
    let mut name_to_id = HashMap::new();
    let mut normalized = vec![];
    let mut start = 0;
    for (i, (&name, (flow_rate, _))) in parsed.iter().enumerate() {
        name_to_id.insert(name, i as u8);
        normalized.push(Valve {
            flow_rate: *flow_rate,
            tunnels: vec![],
        });
        if name == "AA" {
            start = i;
        }
    }
    for (&name, (_, tunnels)) in parsed.iter() {
        let id = name_to_id[name];
        let normalized_tunnels: Vec<_> = tunnels.iter().map(|&tunnel| name_to_id[tunnel]).collect();
        normalized[id as usize].tunnels = normalized_tunnels;
    }
    Input {
        start: start as u8,
        valves: normalized,
    }
}

fn part1(input: &Input) -> i32 {
    let valves = &input.valves;
    let mut max = 0;
    let to_open: u64 = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate != 0)
        .fold(0, |acc, (i, _)| acc | (1 << i));
    let min_distances = min_distances(valves);
    find_most_pressure_to_release(
        valves,
        &min_distances,
        input.start,
        0,
        &mut max,
        30,
        to_open,
    );
    max
}

/// Populates max with the maximum possible amount of pressure which can
/// be released from the given valves over the given amount of time.
fn find_most_pressure_to_release(
    input: &[Valve],
    min_distances: &Grid<u8>,
    curr_valve: u8,
    total_pressure: i32,
    max: &mut i32,
    time_left: u8,
    to_open: u64,
) {
    for n in 0..input.len() {
        if to_open & (1 << n) == 0 {
            continue;
        }
        let d = min_distances[(curr_valve as usize, n as usize)];
        if d >= time_left {
            continue;
        }
        let new_time_left = time_left - d - 1;
        let new_total_pressure =
            total_pressure + (new_time_left as i32) * (input[n].flow_rate as i32);
        if new_total_pressure > *max {
            *max = new_total_pressure;
        }
        find_most_pressure_to_release(
            input,
            min_distances,
            n as u8,
            new_total_pressure,
            max,
            new_time_left,
            to_open & !(1 << n),
        );
    }
}

fn part2(input: &Input) -> i32 {
    let valves = &input.valves;
    let mut max = 0;
    let mut to_open = vec![];
    let min_distances = min_distances(valves);
    for (id, valve) in valves.iter().enumerate() {
        if valve.flow_rate != 0 {
            to_open.push(id);
        }
    }
    // Try partitioning all the valves to open in every possible way, and assign one half of the partition
    // to me and the other half of the partition to the elephant, and then solve those both independently.
    with_all_partitions(&to_open, |assigned_to_me, assigned_to_eleph| {
        let assigned_to_me = assigned_to_me.iter().fold(0, |acc, i| acc | (1 << i));
        let assigned_to_eleph = assigned_to_eleph.iter().fold(0, |acc, i| acc | (1 << i));
        let mut my_best = 0;
        find_most_pressure_to_release(
            valves,
            &min_distances,
            input.start,
            0,
            &mut my_best,
            26,
            assigned_to_me,
        );
        let mut elephant_best = 0;
        find_most_pressure_to_release(
            valves,
            &min_distances,
            input.start,
            0,
            &mut elephant_best,
            26,
            assigned_to_eleph,
        );
        let combined_best = my_best + elephant_best;
        if combined_best > max {
            max = combined_best;
        }
    });
    max
}

/// Invokes f with each possible partition of elems into 2 sets.
fn with_all_partitions<T: Clone>(elems: &[T], mut f: impl FnMut(&[T], &[T])) {
    let mut left = vec![];
    let mut right = vec![];
    partition_helper(elems, &mut left, &mut right, &mut f);
}

fn partition_helper<T: Clone>(
    elems: &[T],
    left: &mut Vec<T>,
    right: &mut Vec<T>,
    f: &mut impl FnMut(&[T], &[T]),
) {
    match elems {
        [] => f(left, right),
        [elem, rest @ ..] => {
            let elem = elem.clone();
            left.push(elem);
            partition_helper(rest, left, right, f);
            let elem = left.pop().unwrap();
            right.push(elem);
            partition_helper(rest, left, right, f);
            right.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 1651);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 1707);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 1871);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 2416);
    }
}

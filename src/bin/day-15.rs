const MY_INPUT: &str = include_str!("../../inputs/day-15.txt");
const TARGET_ROW: i32 = 2000000;
const UPPER_BOUND: i32 = 4000000;

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input, TARGET_ROW));
    println!("part2: {}", part2(&input, UPPER_BOUND));
}

type Input = Vec<((i32, i32), (i32, i32))>;

fn parse_input(s: &str) -> Input {
    fn parse_val(s: &str) -> i32 {
        s.split_once('=').unwrap().1.parse().unwrap()
    }
    fn parse_point(s: &str) -> (i32, i32) {
        let xy = s.split_once(" at ").unwrap().1;
        let (x, y) = xy.split_once(", ").unwrap();
        (parse_val(x), parse_val(y))
    }
    s.lines().map(|line| {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        (parse_point(sensor), parse_point(beacon))
    })
    .collect()
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1-x2).abs() + (y1-y2).abs()
}

fn part1(input: &Input, target_row: i32) -> i32 {
    // In the row where y=2000000, how many positions cannot contain a beacon?
    let mut beacon_free: Vec<(i32, i32)> = vec![];
    for &(sensor, closest_beacon) in input {
        let y_distance_to_target = (target_row-sensor.1).abs();
        let distance_to_beacon = manhattan_distance(sensor, closest_beacon);
        if distance_to_beacon <= y_distance_to_target {
            continue;
        }
        let a = distance_to_beacon - y_distance_to_target;
        let free_range = (sensor.0 - a, sensor.0 + a);
        beacon_free.push(free_range);
    }
    beacon_free.sort_by_key(|&(start, _)| start);
    let mut total = 0;
    let mut prev_end = i32::MIN;
    for (start, end) in beacon_free {
        if prev_end < start {
            total += end - start;
            prev_end = end;
            continue;
        }
        if prev_end >= end {
            continue;
        }
        total += end - prev_end;
        prev_end = end;
    }
    total
}

fn part2(input: &Input, bound: i32) -> i64 {
    for y in 0..=bound {
        if let Some(x) = scan(input, bound, y) {
            return tuning_freq(x, y);
        }
    }
    panic!("o boy");
}

fn tuning_freq(x: i32, y: i32) -> i64 {
    x as i64 * 4000000 + y as i64
}

// returns Some(x), where 0 <= x <= bound if (x, y) could be a beacon, or else
// None if no such x exists.
fn scan(input: &Input, bound: i32, y: i32) -> Option<i32> {
    let mut beacon_free: Vec<(i32, i32)> = vec![];
    for &(sensor, closest_beacon) in input {
        let y_distance_to_target = (y - sensor.1).abs();
        let distance_to_beacon = manhattan_distance(sensor, closest_beacon);
        if distance_to_beacon <= y_distance_to_target {
            continue;
        }
        let a = distance_to_beacon - y_distance_to_target;
        let free_range = (sensor.0 - a, sensor.0 + a);
        beacon_free.push(free_range);
    }
    beacon_free.sort_by_key(|&(start, _)| start);
    let mut prev_end = i32::MIN;
    for (start, end) in beacon_free {
        if prev_end < start - 1 {
            if start > 0 && start <= bound {
                return Some(start - 1);
            }
            if start > bound {
                break;
            }
            prev_end = end;
            continue;
        }
        if prev_end >= end {
            continue;
        }
        prev_end = end;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input, 10), 26);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input, 20), 56000011);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input, TARGET_ROW), 5394423);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input, UPPER_BOUND), 11840879211051);
    }
}

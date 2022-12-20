use std::str::FromStr;

const MY_INPUT: &str = include_str!("../../inputs/day-19.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Blueprint>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ore" | "ore" => Ok(Self::Ore),
            "Clay" | "clay" => Ok(Self::Clay),
            "Obsidian" | "obsidian" => Ok(Self::Obsidian),
            "Geode" | "geode" => Ok(Self::Geode),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    // costs[r] is count of resource r needed to create the robot
    costs: [u8; 4],
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut costs: [u8; 4] = [0; 4];
        let mut words = s.split_ascii_whitespace();
        let count: u8 = words.nth(4).unwrap().parse().unwrap();
        let resource: Resource = words.next().unwrap().parse().unwrap();
        costs[resource as usize] = count;
        while let Some("and") = words.next() {
            let count: u8 = words.next().unwrap().parse().unwrap();
            let resource: Resource = words.next().unwrap().parse().unwrap();
            costs[resource as usize] = count;
        }
        Ok(Robot { costs })
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    // robots[r] is cost to produce robot which generates r.
    robots: [Robot; 4],
}

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let (id, robots) = line.split_once(':').unwrap();
            let id: u8 = id.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
            let mut robots = robots.split('.');
            let ore_robot: Robot = robots.next().unwrap().parse().unwrap();
            let clay_robot: Robot = robots.next().unwrap().parse().unwrap();
            let obsidian_robot: Robot = robots.next().unwrap().parse().unwrap();
            let geode_robot: Robot = robots.next().unwrap().parse().unwrap();
            Blueprint {
                id,
                robots: [ore_robot, clay_robot, obsidian_robot, geode_robot],
            }
        })
        .collect()
}

fn time_to_build(
    robot_costs: &[Robot],
    robots: &[u8],
    resources: &[u32],
    robot_type: Resource,
) -> u8 {
    let robot_type = robot_type as usize;
    robot_costs[robot_type]
        .costs
        .into_iter()
        .enumerate()
        .map(|(resource, resource_cost)| {
            let resource_cost = resource_cost as u32;
            let curr_resource_amt = resources[resource];
            let added_per_turn = robots[resource] as u32;
            if curr_resource_amt >= resource_cost {
                1
            } else if added_per_turn == 0 {
                u8::MAX
            } else {
                let needed = resource_cost - curr_resource_amt;
                // turns = ceil(needed / added_per_turn)
                let turns = (needed + added_per_turn - 1) / added_per_turn;
                assert!(turns < u8::MAX as u32);
                turns as u8 + 1
            }
        })
        .max()
        .unwrap()
}

fn max_geodes(
    robot_costs: &[Robot],
    minutes_remaining: u8,
    robots: &mut [u8],
    resources: &mut [u32],
    max: &mut u32,
) {
    // how many geodes we would create total if we built no more rebots
    let current_heading: u32 = resources[Resource::Geode as usize]
        + minutes_remaining as u32 * robots[Resource::Geode as usize] as u32;
    if current_heading > *max {
        *max = current_heading;
    }
    let n = minutes_remaining as u32;
    let hypothetical_max = current_heading + (n * (n - 1)) / 2;
    if hypothetical_max <= *max {
        return;
    }
    for robot_type in [
        Resource::Clay,
        Resource::Geode,
        Resource::Obsidian,
        Resource::Ore,
    ] {
        if robot_type == Resource::Ore
            && (minutes_remaining - 1)
                <= robot_costs[Resource::Ore as usize].costs[Resource::Ore as usize]
        {
            // don't bother building an ore robot if the amount of ore it would
            // produce over its lifetime is not sufficient to recoup its cost.
            continue;
        }
        let time = time_to_build(robot_costs, robots, resources, robot_type);
        if time >= minutes_remaining {
            continue;
        }
        for i in 0..4 {
            resources[i] += time as u32 * robots[i] as u32;
            resources[i] -= robot_costs[robot_type as usize].costs[i] as u32;
        }
        robots[robot_type as usize] += 1;
        max_geodes(
            robot_costs,
            minutes_remaining - time,
            robots,
            resources,
            max,
        );
        robots[robot_type as usize] -= 1;
        for i in 0..4 {
            resources[i] += robot_costs[robot_type as usize].costs[i] as u32;
            resources[i] -= time as u32 * robots[i] as u32;
        }
    }
}

fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|blueprint| {
            let mut robots = [1, 0, 0, 0];
            let mut resources: [u32; 4] = [0; 4];
            let mut max = 0;
            max_geodes(&blueprint.robots, 24, &mut robots, &mut resources, &mut max);
            blueprint.id as u32 * max
        })
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .iter()
        .take(3)
        .map(|blueprint| {
            let mut robots = [1, 0, 0, 0];
            let mut resources: [u32; 4] = [0; 4];
            let mut max = 0;
            max_geodes(&blueprint.robots, 32, &mut robots, &mut resources, &mut max);
            max
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 1675);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 6840);
    }
}

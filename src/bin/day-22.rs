use std::collections::HashMap;
use std::collections::HashSet;

const MY_INPUT: &str = include_str!("../../inputs/day-22.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Turn {
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    R,
    D,
    L,
    U,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Walk(u8),
    Turn(Turn),
}

#[derive(Debug)]
struct Input {
    map: Map,
    instructions: Vec<Instruction>,
}

fn parse_input(s: &str) -> Input {
    let (map, instructions) = s.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let instructions = parse_instructions(instructions);
    Input { map, instructions }
}

#[derive(Debug)]
struct Map {
    min_row_in_col: Vec<usize>,
    max_row_in_col: Vec<usize>,
    min_col_in_row: Vec<usize>,
    max_col_in_row: Vec<usize>,
    walls: HashSet<(usize, usize)>,
}

fn parse_map(s: &str) -> Map {
    let mut min_row_in_col: HashMap<usize, usize> = HashMap::new();
    let mut max_row_in_col: HashMap<usize, usize> = HashMap::new();
    let mut min_col_in_row = vec![];
    let mut max_col_in_row = vec![];
    let mut walls = HashSet::new();
    for (r, line) in s.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == ' ' {
                continue;
            }
            if char == '#' {
                walls.insert((r, c));
            }
            if min_col_in_row.len() <= r {
                min_col_in_row.push(c);
                max_col_in_row.push(c);
            }
            max_col_in_row[r] = c;
            min_row_in_col.entry(c).or_insert(r);
            max_row_in_col.insert(c, r);
        }
    }
    let min_row_in_col = {
        let mut v = vec![0; min_row_in_col.len()];
        for (c, r) in min_row_in_col {
            v[c] = r;
        }
        v
    };
    let max_row_in_col = {
        let mut v = vec![0; max_row_in_col.len()];
        for (c, r) in max_row_in_col {
            v[c] = r;
        }
        v
    };
    Map {
        min_row_in_col,
        max_row_in_col,
        min_col_in_row,
        max_col_in_row,
        walls,
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut cs = s.chars().peekable();
    let mut instructions = vec![];
    while let Some(c) = cs.next() {
        let mut count = c.to_digit(10).unwrap() as u8;
        loop {
            match cs.peek() {
                Some(c) if c.is_ascii_digit() => {
                    let c = cs.next().unwrap();
                    count = count * 10 + c.to_digit(10).unwrap() as u8;
                }
                _ => break,
            }
        }
        instructions.push(Instruction::Walk(count));
        let dir = match cs.next() {
            Some('L') => Turn::L,
            Some('R') => Turn::R,
            _ => break,
        };
        instructions.push(Instruction::Turn(dir));
    }
    instructions
}

fn next_pos(r: usize, c: usize, dir: Dir, map: &Map) -> (usize, usize) {
    match dir {
        Dir::R => {
            if c < map.max_col_in_row[r] {
                (r, c + 1)
            } else {
                (r, map.min_col_in_row[r])
            }
        }
        Dir::D => {
            if r < map.max_row_in_col[c] {
                (r + 1, c)
            } else {
                (map.min_row_in_col[c], c)
            }
        }
        Dir::L => {
            if c > map.min_col_in_row[r] {
                (r, c - 1)
            } else {
                (r, map.max_col_in_row[r])
            }
        }
        Dir::U => {
            if r > map.min_row_in_col[c] {
                (r - 1, c)
            } else {
                (map.max_row_in_col[c], c)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Face {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn pos_to_face(r: usize, c: usize) -> ((usize, usize), Face) {
    let face_r = r % CUBE_SIZE;
    let face_c = c % CUBE_SIZE;
    let face = match (r / CUBE_SIZE, c / CUBE_SIZE) {
        (0, 1) => Face::A,
        (0, 2) => Face::B,
        (1, 1) => Face::C,
        (2, 0) => Face::D,
        (2, 1) => Face::E,
        (3, 0) => Face::F,
        _ => panic!("bad coords"),
    };
    ((face_r, face_c), face)
}

impl Face {
    fn to_row_col(&self, r: usize, c: usize) -> (usize, usize) {
        match self {
            Face::A => (r, c + CUBE_SIZE),
            Face::B => (r, c + 2 * CUBE_SIZE),
            Face::C => (CUBE_SIZE + r, CUBE_SIZE + c),
            Face::D => (2 * CUBE_SIZE + r, c),
            Face::E => (2 * CUBE_SIZE + r, CUBE_SIZE + c),
            Face::F => (3 * CUBE_SIZE + r, c),
        }
    }
}

const CUBE_SIZE: usize = 50;

fn next_pos_cube(r: usize, c: usize, dir: Dir, map: &Map) -> ((usize, usize), Dir) {
    let curr_face = pos_to_face(r, c);
    match dir {
        Dir::R => {
            if c < map.max_col_in_row[r] {
                ((r, c + 1), dir)
            } else {
                let ((r, c), curr_face) = curr_face;
                match curr_face {
                    Face::A => (Face::B.to_row_col(r, 0), Dir::R),
                    Face::B => (Face::E.to_row_col(CUBE_SIZE - 1 - r, CUBE_SIZE - 1), Dir::L),
                    Face::C => (Face::B.to_row_col(CUBE_SIZE - 1, r), Dir::U),
                    Face::D => (Face::E.to_row_col(r, 0), Dir::R),
                    Face::E => (Face::B.to_row_col(CUBE_SIZE - 1 - r, CUBE_SIZE - 1), Dir::L),
                    Face::F => (Face::E.to_row_col(CUBE_SIZE - 1, r), Dir::U),
                }
            }
        }
        Dir::D => {
            if r < map.max_row_in_col[c] {
                ((r + 1, c), dir)
            } else {
                let ((r, c), curr_face) = curr_face;
                match curr_face {
                    Face::A => (Face::C.to_row_col(0, c), Dir::D),
                    Face::B => (Face::C.to_row_col(c, CUBE_SIZE - 1), Dir::L),
                    Face::C => (Face::E.to_row_col(0, c), Dir::D),
                    Face::D => (Face::F.to_row_col(0, c), Dir::D),
                    Face::E => (Face::F.to_row_col(c, CUBE_SIZE - 1), Dir::L),
                    Face::F => (Face::B.to_row_col(0, c), Dir::D),
                }
            }
        }
        Dir::L => {
            if c > map.min_col_in_row[r] {
                ((r, c - 1), dir)
            } else {
                let ((r, c), curr_face) = curr_face;
                match curr_face {
                    Face::A => (Face::D.to_row_col(CUBE_SIZE - 1 - r, 0), Dir::R),
                    Face::B => (Face::A.to_row_col(r, CUBE_SIZE - 1), Dir::L),
                    Face::C => (Face::D.to_row_col(0, r), Dir::D),
                    Face::D => (Face::A.to_row_col(CUBE_SIZE - 1 - r, 0), Dir::R),
                    Face::E => (Face::D.to_row_col(r, CUBE_SIZE - 1), Dir::L),
                    Face::F => (Face::A.to_row_col(0, r), Dir::D),
                }
            }
        }
        Dir::U => {
            if r > map.min_row_in_col[c] {
                ((r - 1, c), dir)
            } else {
                let ((r, c), curr_face) = curr_face;
                match curr_face {
                    Face::A => (Face::F.to_row_col(c, 0), Dir::R),
                    Face::B => (Face::F.to_row_col(CUBE_SIZE - 1, c), Dir::U),
                    Face::C => (Face::A.to_row_col(CUBE_SIZE - 1, c), Dir::U),
                    Face::D => (Face::C.to_row_col(c, 0), Dir::R),
                    Face::E => (Face::C.to_row_col(CUBE_SIZE - 1, c), Dir::U),
                    Face::F => (Face::D.to_row_col(CUBE_SIZE - 1, c), Dir::U),
                }
            }
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut r = 0;
    let mut c = input.map.min_col_in_row[0];
    while input.map.walls.contains(&(r, c)) {
        c += 1;
    }
    let mut dir = Dir::R;
    for inst in &input.instructions {
        match inst {
            Instruction::Walk(count) => {
                for _ in 0..*count {
                    let (next_r, next_c) = next_pos(r, c, dir, &input.map);
                    if input.map.walls.contains(&(next_r, next_c)) {
                        break;
                    }
                    r = next_r;
                    c = next_c;
                }
            }
            Instruction::Turn(turn) => {
                dir = match (turn, dir) {
                    (Turn::L, Dir::R) => Dir::U,
                    (Turn::L, Dir::D) => Dir::R,
                    (Turn::L, Dir::L) => Dir::D,
                    (Turn::L, Dir::U) => Dir::L,
                    (Turn::R, Dir::R) => Dir::D,
                    (Turn::R, Dir::D) => Dir::L,
                    (Turn::R, Dir::L) => Dir::U,
                    (Turn::R, Dir::U) => Dir::R,
                };
            }
        }
    }
    1000 * (r + 1) + 4 * (c + 1) + dir as usize
}

fn part2(input: &Input) -> usize {
    let mut r = 0;
    let mut c = input.map.min_col_in_row[0];
    while input.map.walls.contains(&(r, c)) {
        c += 1;
    }
    let mut dir = Dir::R;
    for inst in &input.instructions {
        match inst {
            Instruction::Walk(count) => {
                for _ in 0..*count {
                    let ((next_r, next_c), next_dir) = next_pos_cube(r, c, dir, &input.map);
                    if input.map.walls.contains(&(next_r, next_c)) {
                        break;
                    }
                    r = next_r;
                    c = next_c;
                    dir = next_dir;
                }
            }
            Instruction::Turn(turn) => {
                dir = match (turn, dir) {
                    (Turn::L, Dir::R) => Dir::U,
                    (Turn::L, Dir::D) => Dir::R,
                    (Turn::L, Dir::L) => Dir::D,
                    (Turn::L, Dir::U) => Dir::L,
                    (Turn::R, Dir::R) => Dir::D,
                    (Turn::R, Dir::D) => Dir::L,
                    (Turn::R, Dir::L) => Dir::U,
                    (Turn::R, Dir::U) => Dir::R,
                };
            }
        }
    }
    1000 * (r + 1) + 4 * (c + 1) + dir as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 6032);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 117102);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 135297);
    }
}

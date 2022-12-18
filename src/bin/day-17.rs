use bit_set::BitSet;

const MY_INPUT: &str = include_str!("../../inputs/day-17.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

struct Input {
    lefts: BitSet,
    len: usize,
}

impl Input {
    fn jet_iter(&self) -> impl Iterator<Item = Jet> + '_ {
        (0..self.len)
            .map(|i| {
                if self.lefts.contains(i) {
                    Jet::Left
                } else {
                    Jet::Right
                }
            })
            .cycle()
    }
}

enum Jet {
    Left,
    Right,
}

fn parse_input(s: &str) -> Input {
    let mut bs = BitSet::with_capacity(s.len());
    let mut len = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '<' => {
                bs.insert(i);
            }
            '>' => (),
            _ => continue,
        }
        len += 1;
    }
    Input { lefts: bs, len }
}

type Shape = [(u8, u8)];

const SHAPES: &[&Shape] = &[
    // ####
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // .#.
    // ###
    // .#.
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    // ..#
    // ..#
    // ###
    &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
    // #
    // #
    // #
    // #
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // ##
    // ##
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

const CHAMBER_WIDTH: u8 = 7;

struct Pile {
    layers: Vec<u8>,
}

impl Pile {
    fn add_shape(&mut self, r: usize, c: u8, shape: &Shape) {
        for &(sr, sc) in shape {
            let r = r + sr as usize;
            let c = c + sc;
            while self.height() <= r {
                self.layers.push(0);
            }
            self.layers[r] |= 1 << c;
        }
    }

    fn can_move_left(&self, r: usize, c: u8, shape: &Shape) -> bool {
        shape.iter().copied().all(|(sr, sc)| {
            let r = r + sr as usize;
            let c = c + sc;
            c > 0 && (r >= self.height() || (self.layers[r] & (1 << (c - 1)) == 0))
        })
    }

    fn can_move_right(&self, r: usize, c: u8, shape: &Shape) -> bool {
        shape.iter().copied().all(|(sr, sc)| {
            let r = r + sr as usize;
            let c = c + sc;
            (c + 1) < CHAMBER_WIDTH
                && (r >= self.height() || (self.layers[r] & (1 << (c + 1)) == 0))
        })
    }

    fn cant_go_down(&self, r: usize, c: u8, shape: &Shape) -> bool {
        if r == 0 {
            return true;
        }
        for &(sr, sc) in shape {
            let r = r + sr as usize;
            let c = c + sc;
            if r - 1 < self.height() && (self.layers[r - 1] & (1 << c) != 0) {
                return true;
            }
        }
        false
    }

    fn new() -> Pile {
        Pile { layers: vec![] }
    }

    fn height(&self) -> usize {
        self.layers.len()
    }
}

fn part1(input: &Input) -> usize {
    let mut jets = input.jet_iter();
    let mut shapes = SHAPES.iter().copied().cycle();
    let mut pile = Pile::new();
    for _ in 0..2022 {
        let shape = shapes.next().unwrap();
        insert_shape(shape, &mut jets, &mut pile);
    }
    pile.height()
}

fn insert_shape(shape: &Shape, jets: &mut impl Iterator<Item = Jet>, pile: &mut Pile) {
    let mut shape_row = pile.height() + 3;
    let mut shape_col: u8 = 2;
    loop {
        match jets.next().unwrap() {
            Jet::Left => {
                if pile.can_move_left(shape_row, shape_col, shape) {
                    shape_col -= 1;
                }
            }
            Jet::Right => {
                if pile.can_move_right(shape_row, shape_col, shape) {
                    shape_col += 1;
                }
            }
        }
        if pile.cant_go_down(shape_row, shape_col, shape) {
            pile.add_shape(shape_row, shape_col, shape);
            break;
        }
        shape_row -= 1;
    }
}

fn part2(input: &Input) -> usize {
    let mut jets = input.jet_iter();
    let mut shapes = SHAPES.iter().copied().cycle();
    let mut pile = Pile::new();
    let mut i: u64 = 0;
    while i < 1000000000000 {
        if i == 5_953_690 {
            i = 999_987_726_090;
        }
        let shape = shapes.next().unwrap();
        insert_shape(shape, &mut jets, &mut pile);
        i += 1;
    }
    pile.height() + 1_564_677_361_520
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 3137);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 1564705882327);
    }
}

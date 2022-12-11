use bit_set::BitSet;

const MY_INPUT: &str = include_str!("../../inputs/day-11.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Vec<u8>>;

fn parse_input(s: &str) -> Input {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let outside_size = num_rows * 2 + (num_cols - 2) * 2;
    let mut interior_visible = BitSet::with_capacity(num_rows * num_cols);
    let mut interior_visible_count = 0;
    for r in 1..(num_rows - 1) {
        let mut max = input[r][0];
        for c in 1..(num_cols - 1) {
            let tree = input[r][c];
            if tree > max {
                if interior_visible.insert(num_cols * r + c) {
                    interior_visible_count += 1;
                }
                max = tree;
            }
        }
        max = input[r][num_cols - 1];
        for c in (1..(num_cols - 1)).rev() {
            let tree = input[r][c];
            if tree > max {
                if interior_visible.insert(num_cols * r + c) {
                    interior_visible_count += 1;
                }
                max = tree;
            }
        }
    }
    for c in 1..(num_cols - 1) {
        let mut max = input[0][c];
        for r in 1..(num_rows - 1) {
            let tree = input[r][c];
            if tree > max {
                if interior_visible.insert(num_cols * r + c) {
                    interior_visible_count += 1;
                }
                max = tree;
            }
        }
        max = input[num_rows - 1][c];
        for r in (1..(num_rows - 1)).rev() {
            let tree = input[r][c];
            if tree > max {
                if interior_visible.insert(num_cols * r + c) {
                    interior_visible_count += 1;
                }
                max = tree;
            }
        }
    }
    outside_size + interior_visible_count
}

fn part2(input: &Input) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    (0..num_rows)
        .map(|r| {
            (0..num_cols)
                .map(|c| scenic_score(input, r, c))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn scenic_score(input: &Input, r: usize, c: usize) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let tree = input[r][c];
    fn viewing_distance(tree: u8, line_of_sight: &mut dyn Iterator<Item = u8>) -> usize {
        let mut distance = 0;
        for other_tree in line_of_sight {
            distance += 1;
            if tree <= other_tree {
                break;
            }
        }
        distance
    }
    let mut left_line_of_sight = (0..c).rev().map(|c| input[r][c]);
    let mut right_line_of_sight = ((c + 1)..num_cols).map(|c| input[r][c]);
    let mut up_line_of_sight = (0..r).rev().map(|r| input[r][c]);
    let mut down_line_of_sight = ((r + 1)..num_rows).map(|r| input[r][c]);
    let lines_of_sight: [&mut dyn Iterator<Item = u8>; 4] = [
        &mut left_line_of_sight,
        &mut right_line_of_sight,
        &mut up_line_of_sight,
        &mut down_line_of_sight,
    ];
    lines_of_sight
        .into_iter()
        .map(|line_of_sight| viewing_distance(tree, line_of_sight))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 1845);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 230112);
    }
}

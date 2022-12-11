use itertools::Itertools;
use std::str::FromStr;

const MY_INPUT: &str = include_str!("../../inputs/day-10.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2:\n{}", part2(&input));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Inst {
    Noop,
    Addx(i8),
}

impl FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_ascii_whitespace().collect();
        match words[..] {
            ["noop"] => Ok(Self::Noop),
            ["addx", val] => Ok(Self::Addx(val.parse().unwrap())),
            _ => Err(()),
        }
    }
}

type Input = Vec<Inst>;

fn parse_input(s: &str) -> Input {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &Input) -> i32 {
    let mut cycle = 1;
    let mut reg = 1;
    let mut signal_strength = 0;
    for inst in input {
        if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
            signal_strength += cycle * reg;
        }
        match inst {
            Inst::Noop => cycle += 1,
            Inst::Addx(v) => {
                cycle += 1;
                if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
                    signal_strength += cycle * reg;
                }
                reg += *v as i32;
                cycle += 1;
            }
        }
    }
    signal_strength
}

fn set_pixel(cycle: i32, reg: i32, screen: &mut [u8]) {
    let pixel = (cycle - 1) % (40 * 6);
    let pixel_x: i32 = pixel % 40;
    let p = if (pixel_x - reg).abs() <= 1 {
        b'#'
    } else {
        b' '
    };
    screen[pixel as usize] = p;
}

fn part2(input: &Input) -> String {
    let mut cycle = 1;
    let mut reg = 1;
    let mut screen = [b' '; 40 * 6];
    for inst in input {
        set_pixel(cycle, reg, &mut screen);
        match inst {
            Inst::Noop => cycle += 1,
            Inst::Addx(v) => {
                cycle += 1;
                set_pixel(cycle, reg, &mut screen);
                cycle += 1;
                reg += *v as i32;
            }
        }
    }
    show_screen(&screen)
}

fn show_screen(screen: &[u8]) -> String {
    (0..6)
        .map(|r| {
            (0..40)
                .map(|c| screen[r * 40 + c] as char)
                .collect::<String>()
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 13140);
    }

    const PART_2_SAMPLE_OUTPUT: &str = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ";

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(PART_2_SAMPLE_OUTPUT, part2(&input));
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 14360);
    }

    const PART_2_MY_OUTPUT: &str = "###   ##  #  #  ##  #### ###  #### #### 
#  # #  # # #  #  # #    #  # #       # 
###  #    ##   #  # ###  #  # ###    #  
#  # # ## # #  #### #    ###  #     #   
#  # #  # # #  #  # #    # #  #    #    
###   ### #  # #  # #### #  # #### #### ";

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), PART_2_MY_OUTPUT);
    }
}

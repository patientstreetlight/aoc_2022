use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

const MY_INPUT: &str = include_str!("../../inputs/day-21.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Div),
            _ => Err(()),
        }
    }
}

enum Expr<'a> {
    Num(i64),
    Bin(&'a str, Op, &'a str),
}

struct Input<'a> {
    defs: HashMap<&'a str, Expr<'a>>,
}

fn parse_input(s: &str) -> Input {
    let defs = s.lines().map(|line| {
        let (var, rhs) = line.split_once(':').unwrap();
        let mut rhs = rhs.split_ascii_whitespace();
        let a = rhs.next().unwrap();
        let expr = match a.parse::<i64>() {
            Ok(a)  => Expr::Num(a),
            _ => {
                let op = rhs.next().unwrap().parse::<Op>().unwrap();
                let b = rhs.next().unwrap();
                assert!(rhs.next().is_none());
                Expr::Bin(a, op, b)
            }
        };
        (var, expr)
    })
    .collect();
    Input { defs }
}

fn part1(input: &Input) -> i64 {
    let mut resolved: HashMap<&str, i64> = HashMap::new();
    resolve("root", input, &mut resolved)
}

fn resolve<'a>(var: &'a str, input: &Input<'a>, resolved: &mut HashMap<&'a str, i64>) -> i64 {
    if let Some(&val) = resolved.get(var) {
        return val;
    }
    let val = match &input.defs[var] {
        Expr::Num(i) => *i,
        Expr::Bin(a, op, b) => {
            let a = resolve(a, input, resolved);
            let b = resolve(b, input, resolved);
            match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mult => a * b,
                Op::Div => a / b,
            }
        }
    };
    resolved.insert(var, val);
    val
}

// Finds all the variables which var transitively depends upon.
fn depends_on<'a>(var: &'a str, input: &Input<'a>) -> HashSet<&'a str> {
    let mut discovered = HashSet::new();
    let mut frontier = vec![var];
    while let Some(var) = frontier.pop() {
        if let Expr::Bin(a, _, b) = input.defs[var] {
            for dep in [a, b] {
                if discovered.insert(dep) {
                    frontier.push(dep);
                }
            }
        }
    }
    discovered
}

fn part2(input: &Input) -> i64 {
    let (left, right) = match input.defs["root"] {
        Expr::Num(_) => panic!("root resolved to a number"),
        Expr::Bin(l, _, r) => (l, r),
    };
    let left_deps = depends_on(left, input);
    let right_deps = depends_on(right, input);
    let (depends_on_humn, doesnt_depend_on_humn) = if left_deps.contains("humn") {
        (left, right)
    } else {
        assert!(right_deps.contains("humn"));
        (right, left)
    };
    let mut resolved: HashMap<&str, i64> = HashMap::new();
    let target_val = resolve(doesnt_depend_on_humn, input, &mut resolved);
    let resolved = resolved;
    let mut lo = 1;
    match try_humn_val(lo, depends_on_humn, input, &resolved).cmp(&target_val) {
        Ordering::Less => panic!("1 is too high"),
        Ordering::Equal => return lo,
        Ordering::Greater => (),
    }
    let mut hi = lo * 2;
    loop {
        match try_humn_val(hi, depends_on_humn, input, &resolved).cmp(&target_val) {
            Ordering::Greater => hi *= 2,
            Ordering::Equal => return hi,
            Ordering::Less => break,
        }
    }
    // lo < hi
    // f(lo) > target.  f(hi) < target.  Can now binary search between them.
    // some value k, lo < k < hi, exists st f(k) == target
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        match try_humn_val(mid, depends_on_humn, input, &resolved).cmp(&target_val) {
            Ordering::Greater => lo = mid,
            Ordering::Equal => {
                let mut cand = mid;
                while try_humn_val(cand - 1, depends_on_humn, input, &resolved) == target_val {
                    cand -= 1;
                }
                return cand;
            }
            Ordering::Less => hi = mid,
        }
    }
    panic!("binary search failed");
}

fn try_humn_val(val: i64, root: &str, input: &Input, resolved: &HashMap<&str, i64>) -> i64 {
    let mut resolved = resolved.clone();
    resolved.insert("humn", val);
    resolve(root, input, &mut resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 301);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 38731621732448);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 3848301405790);
    }
}

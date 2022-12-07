use std::collections::HashMap;

const MY_INPUT: &str = include_str!("../../inputs/day-07.txt");

fn main() {
    let input = parse_input(MY_INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type Input = Vec<Cmd>;

enum Cmd {
    Cd(&'static str),
    Ls(Vec<Entry>),
}

enum Entry {
    Dir(&'static str),
    File(&'static str, usize),
}

enum Fs {
    Dir(HashMap<&'static str, Fs>),
    File(usize),
}

fn build_fs(input: &Input) -> Fs {
    let mut root_dir = HashMap::new();
    let mut cwd: Vec<&str> = vec![];
    for cmd in input {
        match cmd {
            Cmd::Cd(x) => match *x {
                "/" => cwd.clear(),
                ".." => {
                    cwd.pop();
                }
                _ => {
                    let mut parent = &mut root_dir;
                    for dir in cwd.iter() {
                        let next_parent = parent.get_mut(dir).unwrap();
                        parent = match next_parent {
                            Fs::Dir(children) => children,
                            Fs::File(_) => panic!("file can't be a parent"),
                        }
                    }
                    if !parent.contains_key(x) {
                        parent.insert(x, Fs::Dir(HashMap::new()));
                    }
                    cwd.push(x);
                }
            },
            Cmd::Ls(entries) => {
                let mut parent = &mut root_dir;
                for dir in cwd.iter() {
                    let next_parent = parent.get_mut(dir).unwrap();
                    parent = match next_parent {
                        Fs::Dir(children) => children,
                        Fs::File(_) => panic!("file can't be a parent"),
                    }
                }
                for e in entries {
                    match e {
                        Entry::Dir(name) => {
                            if !parent.contains_key(name) {
                                parent.insert(name, Fs::Dir(HashMap::new()));
                            }
                        }
                        Entry::File(name, size) => {
                            parent.insert(name, Fs::File(*size));
                        }
                    }
                }
            }
        }
    }
    Fs::Dir(root_dir)
}

fn parse_input(s: &'static str) -> Input {
    s.split("$ ")
        .skip(1)
        .map(|cmd| {
            let mut lines = cmd.lines();
            let cmd = lines.next().unwrap();
            let cmd: Vec<_> = cmd.split_ascii_whitespace().collect();
            match cmd[..] {
                ["cd", dir] => Cmd::Cd(dir),
                ["ls"] => {
                    let entries = lines
                        .map(|line| {
                            let (a, b) = line.split_once(' ').unwrap();
                            match a {
                                "dir" => Entry::Dir(b),
                                _ => {
                                    let size = a.parse().unwrap();
                                    Entry::File(b, size)
                                }
                            }
                        })
                        .collect();
                    Cmd::Ls(entries)
                }
                _ => panic!("bad cmd: {:?}", cmd),
            }
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    let fs = build_fs(input);
    let mut total: usize = 0;
    let mut update_total = |size| {
        if size <= 100000 {
            total += size;
        }
    };
    total_size(&fs, &mut update_total);
    total
}

fn total_size(fs: &Fs, with_size: &mut impl FnMut(usize)) -> usize {
    match fs {
        Fs::File(size) => *size,
        Fs::Dir(entries) => {
            let size = entries
                .values()
                .map(|entry| total_size(entry, with_size))
                .sum();
            with_size(size);
            size
        }
    }
}

fn part2(input: &Input) -> usize {
    let fs = build_fs(input);
    let mut sizes = vec![];
    let mut add_size = |size| sizes.push(size);
    let total_used = total_size(&fs, &mut add_size);
    let free = 70000000 - total_used;
    let need_to_reclaim = 30000000 - free;
    sizes
        .into_iter()
        .filter(|size| *size >= need_to_reclaim)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn part2_sample() {
        let input = parse_input(SAMPLE);
        assert_eq!(part2(&input), 24933642);
    }

    #[test]
    fn part1_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part1(&input), 1792222);
    }

    #[test]
    fn part2_my_input() {
        let input = parse_input(MY_INPUT);
        assert_eq!(part2(&input), 1112963);
    }
}

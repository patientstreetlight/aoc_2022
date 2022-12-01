
fn main() {
    let input = include_str!("../../inputs/day-01.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part2(input: &str) -> i32 {
    // sorted lowest to highest
    let mut top_3 = [0, 0, 0, 0];
    fn insert(arr: &mut [i32]) {
        let mut i = 0;
        while i < arr.len() - 1 && arr[i] > arr[i+1] {
            arr.swap(i, i+1);
            i += 1;
        }
        arr[0] = 0;
    }
    for line in input.lines() {
        if line.is_empty() {
            insert(&mut top_3);
        } else {
            top_3[0] += line.parse::<i32>().unwrap();
        }
    }
    insert(&mut top_3);
    top_3.into_iter().sum()
}

fn part1(input: &str) -> i32 {
    let mut max_cals = 0;
    let mut curr_cals = 0;
    for line in input.lines() {
        if line.is_empty() {
            curr_cals = 0;
        } else {
            curr_cals += line.parse::<i32>().unwrap();
            if curr_cals > max_cals {
                max_cals = curr_cals;
            }
        }
    }
    max_cals
}
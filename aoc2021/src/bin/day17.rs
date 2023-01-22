use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day17.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let target = load(input);

    let min_x = minimum_x(target);
    let max_x = target.1;
    let min_n = 1;
    let min_y = target.2;
    let max_y = -target.2;

    let mut best_height: i32 = 0;
    'yloop: for y0 in (min_y..max_y+1).rev() {
        for x0 in min_x..max_x+2 {
            for n in min_n.. {
                match check(x0, y0, n, target) {
                    -1 => {},
                    0 => {
                        best_height = y0;
                        break 'yloop;
                    },
                    1 => {break},
                    _ => {unreachable!()}
                }
            }
        }
    }
    step(best_height, best_height) as i64
}

fn part2(input: &str) -> i64 {
    let target = load(input);

    let min_x = minimum_x(target);
    let max_x = target.1;
    let min_n = 1;
    let min_y = target.2;
    let max_y = -target.2;

    let mut velocities: HashSet<(i32, i32)> = HashSet::new();
    for y0 in (min_y..max_y+1).rev() {
        for x0 in min_x..max_x+2 {
            for n in min_n.. {
                match check(x0, y0, n, target) {
                    -1 => {},
                    0 => {
                        velocities.insert((x0, y0));
                        break;
                    },
                    1 => {break},
                    _ => {unreachable!()}
                }
            }
        }
    }
    velocities.len() as i64
}

fn load(input: &str) -> (i32, i32, i32, i32) {
    let pattern = r"target area: x=(.*)\.\.(.*), y=(.*)\.\.(.*)";
    let rg = Regex::new(pattern).unwrap();
    let target = match rg.captures(input) {
        None => { unreachable!() }
        Some(x) => {
            (
                x.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        }
    };
    target
}

fn step(x0: i32, n: i32) -> i32 {
    return n * (2*x0 + 1 - n) /2
}

fn step_bound(x0: i32, n: i32) -> i32 {
    let n = *vec![x0, n].iter().min().unwrap();
    return n * (2*x0 + 1 - n) /2
}

fn step_bound_last(x0: i32) -> i32 {
    return (x0 * x0 + x0) / 2
}

fn minimum_x(target: (i32, i32, i32, i32)) -> i32 {
    let mut curr_n: i32 = 0;
    loop {
        if step_bound_last(curr_n) >= target.0 {
            break curr_n
        }
        curr_n += 1;
    }
}

fn check(x0: i32, y0:i32, n: i32, target: (i32, i32, i32, i32)) -> i8 {
    let x = step_bound(x0, n);
    let y = step(y0, n);
    if x >= target.0 && x <= target.1 && y >= target.2 && y <= target.3 {
        return 0 // in target
    };
    if x > target.1 || y < target.2 {
        return 1 // out of search bounds
    }
    return -1 // not yet in target nor out of bounds
}

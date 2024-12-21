use std::fs;
use std::time::Instant;

use itertools::Itertools;
    
fn main() {
    let input = fs::read_to_string("data/day02.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let line = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            is_safe(&line)
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let line = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if is_safe(&line) {
                return true;
            }

            for idx_to_remove in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(idx_to_remove);
                if is_safe(&new_line) {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

fn is_safe(input: &Vec<i32>) -> bool {
    let diffs = input
        .iter()
        .tuple_windows()
        .map(|(x, y)| x - y)
        .collect::<Vec<i32>>();

    let same_sign = diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0);
    let contains_zero = diffs.iter().any(|&x| x == 0);
    let contains_abs_gt_3 = diffs.iter().any(|&x| x.abs() > 3);

    same_sign && !contains_zero && !contains_abs_gt_3
}

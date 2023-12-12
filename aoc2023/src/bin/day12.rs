use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use num::abs;

fn main() {
    let input = fs::read_to_string("data/day12.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    parse(input).iter()
        .map(|line| {
            count(line.0, &line.1)
        })
        .sum()
}

fn parse(input: &str) -> Vec<(&str, Vec<u8>)> {
    input.split("\n")
        .map(|line| {
            let mut splitted = line.split(" ");
            let row = splitted.next().unwrap();
            let nums = splitted.next().unwrap().split(",")
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            (row, nums)
        })
        .collect()
}

fn count(row: &str, nums: &Vec<u8>) -> usize {
    // println!("{} {:?}", row, nums);
    if row.len() == 0 && nums.len() == 0 {
        return 1;
    }

    if row.len() == 0 {
        return 0;
    }

    match row.chars().next().unwrap() {
        '.' => count(&row[1..], nums),
        '#' => {
            let l = match nums.get(0) {
                Some(x) => *x as usize,
                None => return 0
            };
            if l > row.len() {
                return 0;
            }
            let taken = &row[..l];
            if taken.chars().any(|c| c == '.') {
                return 0
            }
            let row = &row[l..];
            let nums = nums[1..].iter().map(|x| *x).collect::<Vec<u8>>();

            let next = row.chars().next();
            if next.is_some() && next.unwrap() == '#' {
                return 0
            }
            if next.is_some() && next.unwrap() == '?' {
                let row1 = row.replacen("?", ".", 1);
                return count(row1.as_str(), &nums);
            }
            count(row, &nums)
        },
        '?' => {
            let row1 = row.replacen("?", ".", 1);
            let row2 = row.replacen("?", "#", 1);

            count(row1.as_str(), nums) + count(row2.as_str(), nums)
        },
        _ => unreachable!()
    }
}
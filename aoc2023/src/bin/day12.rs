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
            count_possibilities(line.0, &line.1)
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

fn count_possibilities(row: &str, nums: &Vec<u8>) -> usize {
    if row.contains("?") {
        let row1 = row.replacen("?", ".", 1);
        let row2 = row.replacen("?", "#", 1);

        let mut total = 0;
        if partial_valid(row1.as_str(), nums) {
            total += count_possibilities(row1.as_str(), nums);
        }
        if partial_valid(row2.as_str(), nums) {
            total += count_possibilities(row2.as_str(), nums);
        }
        return total
    }

    match partial_valid(row, nums) {
        true => 1,
        false => 0
    }
}

fn partial_valid(row: &str, nums: &Vec<u8>) -> bool {
    if row.chars()
        .filter(|c| *c != '.')
        .count() < nums.iter().map(|x| *x as usize).sum::<usize>() {
        return false
    }

    let row2 = row.splitn(2, "?").next().unwrap();
    let mut actual_nums = row2.split(".")
        .filter(|part| part.len() > 0)
        .map(|part| part.len() as u8)
        .collect::<Vec<u8>>();
    if row.len() != row2.len() && row2.ends_with("#") {
        actual_nums.pop();
    }

    if actual_nums.len() > nums.len() {
        return false
    }

    let nums = &nums[0..actual_nums.len()];

    let ret = *nums == actual_nums;
    return ret
}
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    let mut counter = CachedCounter {
        cache: HashMap::new(),
    };
    parse(input).iter()
        .map(|line| {
            counter.count(line.0, &line.1)
        })
        .sum()
}


fn part2(input: &str) -> usize {
    let mut counter = CachedCounter {
        cache: HashMap::new(),
    };
    parse(input).iter()
        .map(|(row, nums)| {
            let new_row = Vec::from(&[*row, *row, *row, *row, *row]).join("?");
            let new_nums = (0..nums.len()*5).map(|i| *nums.get(i % nums.len()).unwrap()).collect::<Vec<u8>>();

            (new_row, new_nums)
        })
        .map(|line| {
            counter.count(line.0.as_str(), &line.1)
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

struct CachedCounter {
    cache: HashMap<String, usize>
}

impl CachedCounter {
    fn count(&mut self, row: &str, nums: &Vec<u8>) -> usize {
        let hash = format!("{:?}{:?}", row, nums);

        if let Some(x) = self.cache.get(&hash) {
            return *x
        }

        if row.len() == 0 && nums.len() == 0 {
            return 1;
        }

        if row.len() == 0 {
            return 0;
        }

        match row.chars().next().unwrap() {
            '.' => {
                let ret = self.count(&row[1..], nums);
                self.cache.insert(hash, ret);
                ret
            },
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
                    let ret = self.count(row1.as_str(), &nums);
                    self.cache.insert(hash, ret);
                    return ret
                }
                let ret = self.count(row, &nums);
                self.cache.insert(hash, ret);
                return ret
            },
            '?' => {
                let row1 = row.replacen("?", ".", 1);
                let row2 = row.replacen("?", "#", 1);

                let ret = self.count(row1.as_str(), nums) + self.count(row2.as_str(), nums);
                self.cache.insert(hash, ret);
                ret
            },
            _ => unreachable!()
        }
    }
}


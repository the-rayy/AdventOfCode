use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day11.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    input.split_whitespace().map(|num| {
        let num = num.parse::<u64>().unwrap();
        blink(num, 25)
    }).sum()
}

fn blink(num: u64, times: u32) -> u64 {
    if times == 0 {
        return 1
    }

    if num == 0 {
        return blink(1, times - 1);
    }

    if let Some(x) = split_in_half_if_even_length(num) {
        return blink(x.0, times - 1) + blink(x.1, times - 1);
    }

    return blink(num * 2024, times - 1);
}

fn split_in_half_if_even_length(num: u64) -> Option<(u64, u64)> {
    let num_str = num.to_string();
    let len = num_str.len();
    if len % 2 == 0 {
        let half = len / 2;
        let (left, right) = num_str.split_at(half);
        Some((left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
    } else {
        None
    }
}


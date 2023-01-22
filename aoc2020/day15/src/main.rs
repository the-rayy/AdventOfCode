use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let starters = input.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    solve_for(&starters, 2020)
}

fn part2(input: &str) -> i32 {
    let starters = input.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    solve_for(&starters, 30000000)
}

fn solve_for(starters :&Vec<i32>, target :usize) -> i32 {
    let acc_init = *starters.last().unwrap();
    let mut cache = starters[0 .. starters.len()-1].iter()
        .enumerate()
        .map(|(i, &e)| (e, i + 1))
        .collect::<HashMap<i32, usize>>();
    (starters.len() .. target).fold(acc_init, |last, i| (i - cache.insert(last, i).unwrap_or(i)) as i32)
}

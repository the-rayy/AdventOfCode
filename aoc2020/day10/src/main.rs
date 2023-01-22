use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let chargers = ("0\n".to_owned() + input).split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<i64>>();

    let diffs = (0 .. chargers.len()-1).map(|i| chargers[i+1] - chargers[i])
        .collect::<Vec<i64>>();

    let count1 = diffs.iter().filter(|i| **i == 1).count() as i64;
    let count3 = diffs.iter().filter(|i| **i == 3).count() as i64 + 1;

    count1 * count3
}

fn part2(input: &str) -> i64 {
    let chargers = input.split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<i64>>();

    let mut cache :HashMap<usize, i64> = HashMap::new();
    arrangements_count(&chargers, &mut cache, chargers.len()-1)
}

fn arrangements_count(chargers :&Vec<i64>, cache: &mut HashMap<usize, i64>, to_check :usize) -> i64 {
    match cache.get(&to_check) {
        Some(x) => return *x,
        None => ()
    }
    let mut total_combinations :i64 = 0;
    if [1, 2, 3].contains(&chargers[to_check]) {
        total_combinations += 1;
    }
    for i in decr(to_check, 3) ..= decr(to_check, 1) {
        if [1, 2, 3].contains(&(chargers[to_check] - chargers[i])) {
            total_combinations += arrangements_count(chargers, cache, i)
        }
    }

    cache.insert(to_check, total_combinations);
    total_combinations
}

fn decr(input :usize, decr :usize) -> usize {
    if input <= decr {
        0
    } else {
        input - decr
    }
}

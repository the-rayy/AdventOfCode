use std::cmp::max;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use regex;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day09.txt")
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
    let distances: HashMap::<(&str, &str), usize> = input.split("\n")
        .map(|line| {
            let mut splitted = line.split(" ");
            let city1 = splitted.nth(0).unwrap();
            let city2 = splitted.nth(1).unwrap();
            let dist = splitted.nth(1).unwrap().parse::<usize>().unwrap();
            ((city1, city2), dist)
        })
        .collect();

    let cities: HashSet<&str> = distances.keys()
        .map(|k| [k.0, k.1])
        .flatten()
        .collect();

    cities.iter()
        .permutations(cities.len())
        .map(|route| {
            let mut route = route.clone();
            route.push(route.get(0).unwrap());
            route
        })
        .map(|route| {
           let scores = route.iter()
               .tuple_windows()
               .map(|(&&c1, &&c2)| {
                   match distances.get(&(c1, c2)) {
                       None => distances.get(&(c2, c1)).unwrap(),
                       Some(x) => x
                   }
               });
            scores.clone().sum::<usize>() - scores.max().unwrap()
        })
        .min().unwrap()

}

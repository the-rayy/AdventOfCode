use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use num::abs;

fn main() {
    let input = fs::read_to_string("data/day11.txt")
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
    let galaxies = parse(input);

    let (max_i, max_j) = size_of_the_universe(&galaxies);

    let (empty_columns, empty_rows) = find_emptiness_in_the_universe(&galaxies, max_i, max_j);

    galaxies.iter()
        .combinations(2)
        .map(|pair| {
            let min_i = min(pair[0].0, pair[1].0);
            let max_i = max(pair[0].0, pair[1].0);
            let min_j = min(pair[0].1, pair[1].1);
            let max_j = max(pair[0].1, pair[1].1);

            manhattan(pair[0], pair[1]) +
            empty_columns.iter()
                .filter(|&&i| i > min_i && i < max_i)
                .count() +
            empty_rows.iter()
                .filter(|&&j| j > min_j && j < max_j)
                .count()
        })
        .sum::<usize>()
}


fn part2(input: &str) -> usize {
    let galaxies = parse(input);

    let (max_i, max_j) = size_of_the_universe(&galaxies);

    let (empty_columns, empty_rows) = find_emptiness_in_the_universe(&galaxies, max_i, max_j);

    galaxies.iter()
        .combinations(2)
        .map(|pair| {
            let min_i = min(pair[0].0, pair[1].0);
            let max_i = max(pair[0].0, pair[1].0);
            let min_j = min(pair[0].1, pair[1].1);
            let max_j = max(pair[0].1, pair[1].1);

            manhattan(pair[0], pair[1]) +
            empty_columns.iter()
                .filter(|&&i| i > min_i && i < max_i)
                .count() * (1000000-1) +
            empty_rows.iter()
                .filter(|&&j| j > min_j && j < max_j)
                .count() * (1000000-1)
        })
        .sum::<usize>()
}

fn parse(input: &str) -> HashSet<(usize, usize)> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    ((i, j), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|(_, c)| *c == '#')
        .map(|(pos, _)| pos)
        .collect::<HashSet<(usize, usize)>>()
}

fn size_of_the_universe(galaxies: &HashSet<(usize, usize)>) -> (usize, usize) {
    let max_i = galaxies.iter()
        .map(|pos| pos.0)
        .max()
        .unwrap();

    let max_j = galaxies.iter()
        .map(|pos| pos.0)
        .max()
        .unwrap();
    (max_i, max_j)
}

fn find_emptiness_in_the_universe(galaxies: &HashSet<(usize, usize)>, max_i: usize, max_j: usize) -> (Vec<usize>, Vec<usize>) {
    let empty_columns = (0..max_i).filter(|i| {
        let g = galaxies.iter()
            .filter(|pos| pos.0 == *i)
            .count();
        g == 0
    })
        .collect::<Vec<usize>>();

    let empty_rows = (0..max_j).filter(|j| {
        let g = galaxies.iter()
            .filter(|pos| pos.1 == *j)
            .count();
        g == 0
    })
        .collect::<Vec<usize>>();
    (empty_columns, empty_rows)
}

fn manhattan(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    abs(p1.0 as i64 - p2.0 as i64) as usize + abs(p1.1 as i64 - p2.1 as i64) as usize
}
use itertools::Itertools;
use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day08.txt").expect("Unable to load input file");

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
    let max_x = input.lines().count() as i32 - 1;
    let max_y = input.lines().next().unwrap().len() as i32 - 1;
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .filter(|(_, c)| *c != '.')
        .collect();

    let antennas = grid.values().collect::<HashSet<_>>();

    let mut antinodes = Vec::with_capacity(1500);

    for ant in antennas {
        grid.iter()
            .filter(|(_, &c)| c == *ant)
            .map(|(pos, _)| pos)
            .combinations(2)
            .for_each(|pair| {
                let dist = ((pair[1].0 - pair[0].0), (pair[1].1 - pair[0].1));

                let antinode = (pair[0].0 - dist.0, pair[0].1 - dist.1);
                if !(antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_x || antinode.1 > max_y) {
                    antinodes.push(antinode);
                }

                let antinode = (pair[0].0 + 2 * dist.0, pair[0].1 + 2 * dist.1);
                if !(antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_x || antinode.1 > max_y) {
                    antinodes.push(antinode);
                }
            });
    }

    let antinodes = antinodes.iter().collect::<HashSet<_>>();
    antinodes.len() as u32
}

fn part2(input: &str) -> u32 {
    let max_x = input.lines().count() as i32 - 1;
    let max_y = input.lines().next().unwrap().len() as i32 - 1;
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .filter(|(_, c)| *c != '.')
        .collect();

    let antennas = grid.values().collect::<HashSet<_>>();

    let mut antinodes = Vec::with_capacity(1500);

    for ant in antennas {
        grid.iter()
            .filter(|(_, &c)| c == *ant)
            .map(|(pos, _)| pos)
            .combinations(2)
            .for_each(|pair| {
                let dist = ((pair[1].0 - pair[0].0), (pair[1].1 - pair[0].1));

                let mut n = 0;
                loop {
                    let antinode = (pair[0].0 - n * dist.0, pair[0].1 - n * dist.1);

                    if antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_x || antinode.1 > max_y
                    {
                        break;
                    }

                    antinodes.push(antinode);
                    n += 1;
                }

                let mut n = 0;
                loop {
                    let antinode = (pair[0].0 + n * dist.0, pair[0].1 + n * dist.1);

                    if antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_x || antinode.1 > max_y
                    {
                        break;
                    }

                    antinodes.push(antinode);
                    n += 1;
                }
            });
    }

    let antinodes = antinodes.iter().collect::<HashSet<_>>();
    antinodes.len() as u32
}

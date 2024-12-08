use std::fs;
use std::time::Instant;
use itertools::Itertools;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day08.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);
    //
    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let antennas = grid.iter().filter(|(_, &c)| c != '.').map(|(_, &c)| c).collect::<HashSet<_>>();

    let mut antinodes = HashSet::new();

    for ant in antennas {
        grid.iter().filter(|(_, &c)| c == ant).map(|(pos, _)| pos).combinations(2).for_each(|pair| {

            let dist = ((pair[1].0 - pair[0].0), (pair[1].1 - pair[0].1));
            let antinode1 = (pair[0].0 - dist.0, pair[0].1 - dist.1);
            let antinode2 = (pair[0].0 + 2*dist.0, pair[0].1 + 2*dist.1);

            antinodes.insert(antinode1);
            antinodes.insert(antinode2);
        });

    };

    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();

    antinodes.iter().filter(|(x, y)| x >= &0 && y >= &0 && x <= max_x && y <= max_y).count() as u32

}


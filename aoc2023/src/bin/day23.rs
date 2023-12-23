use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env::current_exe;
use std::fs;
use std::ops::Sub;
use std::time::Instant;
use num::integer::lcm;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day23.txt")
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
    let (grid, dims) = parse(input);

    let start = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == 0)
        .next().unwrap().0;

    let target = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == dims.0)
        .next().unwrap().0;

    let mut visited = HashSet::new();
    longest_possible_path(&grid, start, target, visited)

}

fn parse(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize)) {
    let grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    ((i as isize, j as isize), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(isize, isize), char>>();

    let dims = (*grid.keys().map(|(i, _)| i).max().unwrap(), *grid.keys().map(|(_, j)| j).max().unwrap());

    (grid, dims)
}

//caching would be nice here too
fn longest_possible_path(grid: &HashMap<(isize, isize), char>, pos: (isize, isize), target: (isize, isize), visited: HashSet<(isize, isize)>) -> usize {
    let mut visited = visited.clone();
    visited.insert(pos);

    let dirs = match grid[&pos] {
        '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        _ => unreachable!()
    };

    dirs.iter()
        .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
        .filter(|&next_pos| grid.contains_key(&next_pos) && grid[&next_pos] != '#' && !visited.contains(&next_pos))
        .map(|next_pos| {
            if next_pos == target {
                visited.len()
            } else {
                longest_possible_path(grid, next_pos, target, visited.clone())
            }
        })
        .max()
        .unwrap_or(0)
}

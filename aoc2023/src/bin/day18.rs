use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day18.txt")
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
    let instructions = input.split("\n")
        .map(|line| {
            let mut splitted = line.split(" ");
            let dir = splitted.next().unwrap().chars().next().unwrap();
            let val = splitted.next().unwrap().parse::<u64>().unwrap();
            let color = splitted.next().unwrap();
            (dir, val, color)
        })
        .collect::<Vec<(char, u64, &str)>>();

    let mut grid = HashSet::<(i64, i64)>::new();

    let mut pos = (0,0);
    grid.insert(pos);

    for (dir, val, _) in instructions {
        for _ in 0..val {
            let dir = match dir {
                'R' => (0, 1),
                'L' => (0, -1),
                'U' => (1, 0),
                'D' => (-1, 0),
                _ => unreachable!()
            };
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            grid.insert(pos);
        }
    }

    let min_i = *grid.iter().map(|(x, _)| x).min().unwrap()-1;
    let max_i = *grid.iter().map(|(x, _)| x).max().unwrap()+1;
    let min_j = *grid.iter().map(|(_, y)| y).min().unwrap()-1;
    let max_j = *grid.iter().map(|(_, y)| y).max().unwrap()+1;

    let mut outer_boundary = HashSet::<(i64, i64)>::new();
    let mut queue = Vec::<(i64, i64)>::new();
    queue.push((min_i-1, min_j-1));
    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        if outer_boundary.contains(&(i, j)) {
            continue;
        }
        if i < min_i-1 || j < min_j-1 || i >= max_i+1 || j >= max_j+1 {
            continue
        }
        outer_boundary.insert((i, j));
        if !grid.contains(&(i+1, j)) {
            queue.push((i+1, j));
        }
        if !grid.contains(&(i-1, j)) {
            queue.push((i-1, j));
        }
        if !grid.contains(&(i, j+1)) {
            queue.push((i, j+1));
        }
        if !grid.contains(&(i, j-1)) {
            queue.push((i, j-1));
        }
    }
    ((max_i+2-min_i) * (max_j+2-min_j) - outer_boundary.len() as i64) as usize

}


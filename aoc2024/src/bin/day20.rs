use std::time::Instant;
use std::{collections::VecDeque, fs};
use rayon::prelude::*;

use hashbrown::{HashMap, HashSet};
    
fn main() {
    let input = fs::read_to_string("data/day20.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input, part1_ans);
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
    let start = grid.iter().find(|(_, v)| **v == 'S').unwrap().0.clone();
    let end = grid.iter().find(|(_, v)| **v == 'E').unwrap().0.clone();

    let mut dists_to_end = HashMap::new();
    dists_to_end.insert(end, 0);
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((pos, dist)) = queue.pop_front() {
        for dir in dirs.iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(x) = grid.get(&new_pos) {
                if *x != '#' {
                    if !dists_to_end.contains_key(&new_pos) {
                        dists_to_end.insert(new_pos, dist + 1);
                        queue.push_back((new_pos, dist + 1));
                    }
                }
            }
        }
    };

    let shortest = shortest_path(&grid, start);

    let mut saves = vec![];
    for pos in shortest.iter() {
        //for dir in [(1, 1), (1, -1), (-1, 1), (-1, -1), (2, 0), (-2, 0), (0, 2), (0, -2)].iter() {
        for dir in points_in_manhattan_radius(2).iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            let old_dist = dists_to_end.get(pos).unwrap();
            if let Some(new_dist) = dists_to_end.get(&new_pos) {
                if *old_dist > *new_dist + 2 {
                    let save = old_dist - new_dist - 2;
                    saves.push(save);
                }
            }

        }
    };

    saves.iter().filter(|x| **x >= 100).count() as u32
}

fn shortest_path(grid: &HashMap<(i32, i32), char>, start: (i32, i32)) -> Vec<(i32, i32)> {
    let end = grid.iter().find(|(_, v)| **v == 'E').unwrap().0.clone();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back((start, vec![start]));
    visited.insert(start);

    while let Some((pos, path)) = queue.pop_front() {
        if pos == end {
            return path;
        }
        for dir in dirs {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(x) = grid.get(&new_pos) {
                if *x != '#' {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    if !visited.contains(&new_pos) {
                        visited.insert(new_pos);
                        queue.push_back((new_pos, new_path));
                    }
                }
            }
        }
    };

    unreachable!();
}

fn points_in_manhattan_radius(radius: i32) -> Vec<(i32, i32)> { 
    let mut points = Vec::new(); 
    for x in -radius..=radius { 
        let y_range = radius - x.abs(); 
        for y in -y_range..=y_range { 
            points.push((x, y)); 
        } 
    } 
 
    points 
} 


use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day14.txt")
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
    let mut grid = input.split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim_i = grid.len();
    let dim_j = grid.get(0).unwrap().len();

    tiltN(&mut grid, dim_i, dim_j);

    score(&grid, dim_i)
}


fn part2(input: &str) -> usize {
    let mut grid = input.split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim_i = grid.len();
    let dim_j = grid.get(0).unwrap().len();

    let mut cache = HashMap::<u64, (usize, usize)>::new();

    let mut cycle_len = 0;
    let mut cycle_start = 0;

    let mut i = 0;
    loop {
        let mut hash = DefaultHasher::new();
        grid.hash(&mut hash);

        if cache.contains_key(&hash.finish()) {
            cycle_len = i - cache.get(&hash.finish()).unwrap().0;
            cycle_start = cache.get(&hash.finish()).unwrap().0;
            break;
        }

        tiltN(&mut grid, dim_i, dim_j);
        tiltW(&mut grid, dim_i, dim_j);
        tiltS(&mut grid, dim_i, dim_j);
        tiltE(&mut grid, dim_i, dim_j);

        cache.insert(hash.finish(), (i, score(&grid, dim_i)));
        i += 1;
    }
    let target = cycle_start - 1 + (1_000_000_000 - cycle_start) % cycle_len;

    cache.iter().find(|(_, (i, _))| *i == target).unwrap().1.1
}

fn tiltN(grid: &mut Vec<Vec<char>>, dim_i: usize, dim_j: usize) {
    let dir = (-1, 0);

    for i in 0..dim_i {
        for j in 0..dim_j {
            tilt(grid, dir, i, j);
        }
    }
}

fn tiltS(grid: &mut Vec<Vec<char>>, dim_i: usize, dim_j: usize) {
    let dir = (1, 0);

    for i in (0..dim_i).rev() {
        for j in 0..dim_j {
            tilt(grid, dir, i, j);
        }
    }
}

fn tiltW(grid: &mut Vec<Vec<char>>, dim_i: usize, dim_j: usize) {
    let dir = (0, -1);

    for j in 0..dim_j {
        for i in 0..dim_i {
            tilt(grid, dir, i, j);
        }
    }
}

fn tiltE(grid: &mut Vec<Vec<char>>, dim_i: usize, dim_j: usize) {
    let dir = (0, 1);

    for j in (0..dim_j).rev() {
        for i in 0..dim_i {
            tilt(grid, dir, i, j);
        }
    }
}

fn tilt(grid: &mut Vec<Vec<char>>, dir: (i32, i32), i: usize, j: usize) {
    if get(&grid, (i, j)) != 'O' {
        return;
    }

    let new_pos = tilt_rock(&grid, (i, j), dir);
    *grid.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
    *grid.get_mut(new_pos.0).unwrap().get_mut(new_pos.1).unwrap() = 'O';
}

fn get(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> char {
    grid.get(pos.0).unwrap().get(pos.1).unwrap().clone()
}

fn tilt_rock(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32)) -> (usize, usize) {
    let mut pos = pos;

    loop {
        let next = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if next.0 < 0 || next.1 < 0 || next.0 >= grid.len() as i32 || next.1 >= grid.get(0).unwrap().len() as i32 {
            break;
        }
        let next = (next.0 as usize, next.1 as usize);
        if get(grid, next) == '.' {
            pos = next;
        } else {
            break;
        }
    }

    return pos;
}

fn score(grid: &Vec<Vec<char>>, dim_i: usize) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|c| **c == 'O').count() * (dim_i - i)
        })
        .sum()
}
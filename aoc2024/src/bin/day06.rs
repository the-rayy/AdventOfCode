use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day06.txt").expect("Unable to load input file");

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

    let mut pos = grid.iter().find(|(_, &c)| c == '^').unwrap().0.clone();

    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    let mut visited = Vec::with_capacity(1000);
    loop {
        let next = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        match grid.get(&next) {
            Some(&'.') | Some(&'^') => {
                visited.push(pos);
                pos = next;
            }
            Some(&'#') => {
                dir = (dir + 1) % dirs.len();
            }
            Some(_) => {
                unreachable!()
            }
            None => {
                visited.push(pos);
                break;
            }
        }
    }

    visited.sort();
    visited.dedup();
    visited.len() as u32
}

fn part2(input: &str) -> u32 {
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

    let mut pos = grid.iter().find(|(_, &c)| c == '^').unwrap().0.clone();

    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    let mut visited = Vec::with_capacity(1000);
    let mut obstacles = Vec::with_capacity(1000);

    loop {
        let next = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);

        if visited.iter().filter(|(p, _)| p == &next).count() == 0 {
            let mut new_grid = grid.clone();
            new_grid.insert(next, '#');
            if is_loop(&new_grid, &dirs, dir, pos, visited.clone()) {
                obstacles.push(next);
            }
        }

        match grid.get(&next) {
            Some(&'.') | Some(&'^') => {
                visited.push((pos, dir));
                pos = next;
            }
            Some(&'#') => {
                dir = (dir + 1) % dirs.len();
            }
            Some(_) => {
                unreachable!()
            }
            None => {
                visited.push((pos, dir));
                break;
            }
        }
    }

    obstacles.sort();
    obstacles.dedup();
    obstacles.len() as u32
}

fn is_loop(
    grid: &HashMap<(i32, i32), char>,
    dirs: &Vec<(i32, i32)>,
    dir: usize,
    pos: (i32, i32),
    visited: Vec<((i32, i32), usize)>,
) -> bool {
    let mut dir = dir;
    let mut pos = pos;
    let mut visited = visited.clone();

    loop {
        let next = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);

        if visited.contains(&(pos, dir)) {
            return true;
        }

        match grid.get(&next) {
            Some(&'.') | Some(&'^') => {
                visited.push((pos, dir));
                pos = next;
            }
            Some(&'#') => {
                dir = (dir + 1) % dirs.len();
            }
            Some(_) => {
                unreachable!()
            }
            None => {
                return false;
            }
        }
    }
}

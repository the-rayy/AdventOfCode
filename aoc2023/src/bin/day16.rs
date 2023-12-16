use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
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
    let (grid, grid_max_i, grid_max_j) = parse(input);
    let mut solver = Solver::new(grid.clone(), grid_max_i, grid_max_j);
    solver.solve((0, 0), RIGHT)
}


fn part2(input: &str) -> usize {
    let (grid, grid_max_i, grid_max_j) = parse(input);

    let mut options = Vec::<((i32, i32), (i32, i32))>::new();
    for i in 0..grid_max_i {
        options.push(((i, 0), RIGHT));
        options.push(((i, grid_max_j), LEFT));
    }
    for j in 0..grid_max_j {
        options.push(((0, j), DOWN));
        options.push(((grid_max_i, j), UP));
    }

    options.par_iter()
        .map(|(pos, dir)| {
            let mut solver = Solver::new(grid.clone(), grid_max_i, grid_max_j);
            solver.solve(*pos, *dir)
        })
        .max()
        .unwrap()
}

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

fn parse(input: &str) -> (HashMap<(i32, i32), char>, i32, i32) {
    let grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i as i32, j as i32), c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(i32, i32), char>>();

    let grid_max_i = grid.keys().map(|(x, _)| x).max().unwrap().clone();
    let grid_max_j = grid.keys().map(|(_, y)| y).max().unwrap().clone();

    return (grid, grid_max_i, grid_max_j);
}

struct Solver {
    grid: HashMap<(i32, i32), char>,
    grid_max_i: i32,
    grid_max_j: i32,
    energized: HashSet<((i32, i32), (i32, i32))>,
    next: Vec<((i32, i32), (i32, i32))>,
}

impl Solver {
    fn new(grid: HashMap<(i32, i32), char>, grid_max_i: i32, grid_max_j: i32) -> Solver {
        Solver{
            grid,
            grid_max_i,
            grid_max_j,
            energized: HashSet::new(),
            next: Vec::new(),
        }
    }

    fn solve(&mut self, start_pos: (i32, i32), start_dir: (i32, i32)) -> usize {
        self.push(start_pos, start_dir);

        while let Some((pos, dir)) = self.next.pop() {

            match self.grid.get(&pos) {
                Some('.') => self.push((pos.0 + dir.0, pos.1 + dir.1), dir),
                Some('/') => {
                    let new_dir = (-dir.1, -dir.0);
                    self.push((pos.0 + new_dir.0, pos.1 + new_dir.1), new_dir);
                },
                Some('\\') => {
                    let new_dir = (dir.1, dir.0);
                    self.push((pos.0 + new_dir.0, pos.1 + new_dir.1), new_dir);
                },
                Some('|') => {
                    if dir.0 == 0 {
                        self.push((pos.0 + UP.0, pos.1 + UP.1), UP);
                        self.push((pos.0 + DOWN.0, pos.1 + DOWN.1), DOWN);
                    } else {
                        self.push((pos.0 + dir.0, pos.1 + dir.1), dir);
                    }
                }
                Some('-') => {
                    if dir.1 == 0 {
                        self.push((pos.0 + LEFT.0, pos.1 + LEFT.1), LEFT);
                        self.push((pos.0 + RIGHT.0, pos.1 + RIGHT.1), RIGHT);
                    } else {
                        self.push((pos.0 + dir.0, pos.1 + dir.1), dir);
                    }
                }

                Some(_) => unreachable!(),
                None => {}
            }
        };

        self.energized.iter()
            .map(|(pos, _)| *pos)
            .collect::<HashSet<(i32, i32)>>()
            .len()
    }

    fn push(&mut self, pos: (i32, i32), dir: (i32, i32)) {
        if self.energized.contains(&(pos, dir)) {
            return;
        }

        if pos.0 < 0 || pos.1 < 0 || pos.0 > self.grid_max_i || pos.1 > self.grid_max_j {
            return;
        }

        self.energized.insert((pos, dir));
        self.next.push((pos, dir));
    }
}
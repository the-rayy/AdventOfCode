use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day17.txt")
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


fn part1(input: &str) -> u64 {
    let (grid, grid_max_i, grid_max_j) = parse(input);
    let mut solver = Solver::new(grid.clone(), grid_max_i, grid_max_j);

    solver.solve((0, 0), (grid_max_i, grid_max_j))
}

fn parse(input: &str) -> (HashMap<(i32, i32), u64>, i32, i32) {
    let grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i as i32, j as i32), c.to_string().parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(i32, i32), u64>>();

    let grid_max_i = grid.keys().map(|(x, _)| x).max().unwrap().clone();
    let grid_max_j = grid.keys().map(|(_, y)| y).max().unwrap().clone();

    return (grid, grid_max_i, grid_max_j);
}

struct Solver {
    grid: HashMap<(i32, i32), u64>,
    grid_max_i: i32,
    grid_max_j: i32,
    next: BinaryHeap<(i64, (i32, i32), (i32, i32), u8)>,
    scores: HashMap<((i32, i32), (i32, i32)), (u64, u8)>
}

impl Solver {
    fn new(grid: HashMap<(i32, i32), u64>, grid_max_i: i32, grid_max_j: i32) -> Self {
        Self {
            grid,
            grid_max_i,
            grid_max_j,
            next: BinaryHeap::new(),
            scores: HashMap::new(),
        }
    }

    fn solve(&mut self, start: (i32, i32), target: (i32, i32)) -> u64 {
        self.push(start, 0, (0, 0), 0);

        while let Some((score, pos, last_dir, last_dir_count)) = self.next.pop() {
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            let score = -score as u64;

            for dir in dirs {
                if dir == last_dir && last_dir_count == 3 {
                    continue;
                }
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                let (new_dir, new_dir_count) = if dir == last_dir {
                    (last_dir, last_dir_count + 1)
                } else {
                    (dir, 1)
                };

                match self.grid.get(&new_pos) {
                    Some(new_score) => {
                        let new_score = *new_score + score;
                        self.push(new_pos, new_score, new_dir, new_dir_count);
                        if new_pos == target {
                            return new_score;
                        }
                    },
                    None => {}
                }
            }
        }
        unreachable!()
    }

    fn push(&mut self, pos: (i32, i32), score: u64, dir: (i32, i32), dir_count: u8) {
        if pos.0 < 0 || pos.1 < 0 || pos.0 > self.grid_max_i || pos.1 > self.grid_max_j {
            return;
        }

        if let Some((old_score, old_dir_count)) = self.scores.get(&(pos, dir)) {
            if dir_count >= *old_dir_count && score >= *old_score {
                return;
            }
        }

        self.scores.insert((pos, dir), (score, dir_count));
        self.next.push((-(score as i64), pos, dir, dir_count));
    }
}
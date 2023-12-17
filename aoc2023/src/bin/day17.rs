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
    // next: Vec<((i32, i32), u64, [(i32, i32); 3])>,
    next: BinaryHeap<(i64, (i32, i32), [(i32, i32); 3])>,
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
        self.push(start, 0, [(0, 0); 3]);

        while let Some((score, pos, dir_history)) = self.next.pop() {
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            let score = -score as u64;

            for dir in dirs {
                if dir_history[2] == (-dir.0, -dir.1) {
                    continue;
                }
                if dir == dir_history[0] && dir == dir_history[1] && dir == dir_history[2] {
                    continue;
                }
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                let new_dirs = [dir_history[1], dir_history[2], dir];

                match self.grid.get(&new_pos) {
                    Some(new_score) => {
                        let new_score = *new_score + score;
                        self.push(new_pos, new_score, new_dirs);
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

    fn push(&mut self, pos: (i32, i32), score: u64, dirs: [(i32, i32); 3]) {
        if pos.0 < 0 || pos.1 < 0 || pos.0 > self.grid_max_i || pos.1 > self.grid_max_j {
            return;
        }

        let (dir, count) = foo(dirs);

        if let Some((old_score, old_dir_count)) = self.scores.get(&(pos, dir)) {
            if count >= *old_dir_count && score >= *old_score {
                return;
            }
        }

        self.scores.insert((pos, dir), (score, count));
        self.next.push((-(score as i64), pos, dirs));
    }
}

fn foo(dirs: [(i32, i32); 3]) -> ((i32, i32), u8) {
    let d = dirs[2];
    if dirs[1] != dirs[2] {
        return (d, 1);
    }
    if dirs[0] != dirs[1] {
        return (d, 2);
    }
    return (d, 3);
}
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Component::ParentDir;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day10.txt")
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
    let mut solver = Solver::new(input);

    solver.solve()

}


fn part2(input: &str) -> usize {
    let mut solver = Solver::new(input);

    solver.solve2()

}

#[derive(Debug)]
struct Solver {
    grid: HashMap<(usize, usize), char>,
    distances: HashMap<(usize, usize), usize>,
    tovisit: Vec<((usize, usize), usize)>
}

impl Solver {
    fn new(input: &str) -> Solver {
        let grid = input.split("\n")
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| ((i, j), c))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<HashMap<(usize, usize), char>>();

        Solver{
            grid,
            distances: HashMap::new(),
            tovisit: Vec::new()
        }
    }

    fn solve2(&mut self) -> usize {
        self.solve();

        let max_i = self.grid.keys()
            .map(|k| k.0)
            .max()
            .unwrap();

        let max_j = self.grid.keys()
            .map(|k| k.1)
            .max()
            .unwrap();

        let mut total = 0;
        for i in 0..max_i {
            let mut counter = 0;
            let mut pipe_last = ' ';
            for j in 0..max_j {
                match (self.distances.contains_key(&(i, j)), self.grid.get(&(i, j))) {
                    (false, _) => if counter % 2 == 1 {
                        total += 1;
                    }
                    (true, Some(x)) => {
                        match (x, pipe_last) {
                            ('|', _) => counter += 1,
                            ('S', _) => pipe_last = 'L',
                            ('L', _) | ('F', _) => pipe_last = *x,
                            ('7', 'L') | ('J', 'F') => counter +=1,
                            _ => {}
                        }
                    }
                    _ => unreachable!()
                }
            }
        }

        total
    }

    fn solve(&mut self) -> usize {
        let start = self.grid.iter()
            .filter(|(_, v)| {
            **v == 'S'
        })
            .map(|(&k, _)| k)
            .next()
            .unwrap();

        self.distances.insert(start, 0);

        let main_loop = self.find_main_loop_start(&start);

        main_loop.iter()
            .for_each(|k| {
                self.tovisit.push((*k, 0));
            });

        while !self.tovisit.is_empty() {
            let (point, depth) = self.tovisit.pop().unwrap();
            self.distances.insert(point, depth+1);
            let neigh = self.neighbours(point);
            if !self.distances.contains_key(&neigh[0]) {
                self.tovisit.insert(0, (neigh[0], depth+1))
            }
            if !self.distances.contains_key(&neigh[1]) {
                self.tovisit.insert(0, (neigh[1], depth+1))
            }
        }

        *self.distances.values().max().unwrap()
    }

    fn find_main_loop_start(&mut self, start: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut main_loop = Vec::<(usize, usize)>::new();
        if let Some(down) = self.grid.get(&(start.0 + 1, start.1)) {
            if ['|', 'L', 'J'].contains(&down) {
                main_loop.push((start.0 + 1, start.1))
            }
        }
        if let Some(down) = self.grid.get(&(start.0 - 1, start.1)) {
            if ['|', '7', 'F'].contains(&down) {
                main_loop.push((start.0 - 1, start.1))
            }
        }
        if let Some(down) = self.grid.get(&(start.0, start.1 - 1)) {
            if ['-', 'L', 'F'].contains(&down) {
                main_loop.push((start.0, start.1 - 1))
            }
        }
        if let Some(down) = self.grid.get(&(start.0, start.1 + 1)) {
            if ['-', 'J', '7'].contains(&down) {
                main_loop.push((start.0, start.1 + 1))
            }
        }
        main_loop
    }

    fn neighbours(&self, point: (usize, usize)) -> [(usize, usize); 2] {
        let pipe = *self.grid.get(&point).unwrap();
        match pipe {
            '|' => [
                (point.0+1, point.1),
                (point.0-1, point.1),
            ],
            '-' => [
                (point.0, point.1+1),
                (point.0, point.1-1),
            ],
            'L' => [
                (point.0-1, point.1),
                (point.0, point.1+1),
            ],
            'J' => [
                (point.0-1, point.1),
                (point.0, point.1-1),
            ],
            '7' => [
                (point.0+1, point.1),
                (point.0, point.1-1),
            ],
            'F' => [
                (point.0+1, point.1),
                (point.0, point.1+1),
            ],
            _ => unreachable!()
        }
    }
}


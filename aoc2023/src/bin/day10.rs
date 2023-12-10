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
        }
    }

    fn solve2(&mut self) -> usize {
        let (s_pos, s_val) = self.what_s_should_be();
        self.grid.insert(s_pos, s_val);

        let main_loop = self.find_loop(s_pos)
            .into_iter()
            .collect::<HashSet<(usize, usize)>>();

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
                match (main_loop.contains(&(i, j)), self.grid.get(&(i, j))) {
                    (false, _) => if counter % 2 == 1 {
                        total += 1;
                    }
                    (true, Some(x)) => {
                        match (pipe_last, x) {
                            (_, '|') | ('L', '7') | ('F', 'J') => counter += 1,
                            (_, 'L') | (_, 'F') => pipe_last = *x,
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
        let (s_pos, s_val) = self.what_s_should_be();
        self.grid.insert(s_pos, s_val);

        let main_loop = self.find_loop(s_pos);

        main_loop.len() / 2
    }

    fn find_loop(&mut self, s_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let s_neighbours = self.neighbours(s_pos);

        let mut main_loop = Vec::<(usize, usize)>::new();
        main_loop.push(s_pos);
        main_loop.push(s_neighbours[0]);

        loop {
            let current = *main_loop.last().unwrap();
            let previous = *main_loop.get(main_loop.len() - 2).unwrap();
            let next = *self.neighbours(current).iter()
                .filter(|candidate| **candidate != previous)
                .next()
                .unwrap();
            if next == s_pos {
                break;
            }
            main_loop.push(next);
        }
        main_loop
    }

    fn what_s_should_be(&self) -> ((usize, usize), char) {
        let pos = self.grid.iter()
            .filter(|(_, v)| {
                **v == 'S'
            })
            .map(|(&k, _)| k)
            .next()
            .unwrap();

        let up = self.grid.get(&(pos.0-1, pos.1)).unwrap();
        let down = self.grid.get(&(pos.0+1, pos.1)).unwrap();
        let left = self.grid.get(&(pos.0, pos.1-1)).unwrap();
        let right = self.grid.get(&(pos.0, pos.1+1)).unwrap();

        let up = ['|', 'F', '7'].contains(up);
        let down = ['|', 'L', 'J'].contains(down);
        let left = ['-', 'L', 'F'].contains(left);
        let right = ['-', '7', 'J'].contains(right);

        let s = match (up, down, left, right) {
            (true, true, _, _) => '|',
            (true, _, true, _) => 'J',
            (true, _, _, true) => 'L',
            (_, true, true, _) => '7',
            (_, true, _, true) => 'F',
            (_, _, true, true) => '-',
            _ => unreachable!()
        };

        (pos, s)
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


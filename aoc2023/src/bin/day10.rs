use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day10.txt")
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
    let mut solver = Solver::new(input);

    println!("{:?}", solver);

    solver.solve()

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

        println!("{:?}", self.distances);

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


use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day06.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

//    let part2_start = Instant::now();
//    let part2_ans = part2(&input);
//    println!("Part 2 time: {:.2?}", part2_start.elapsed());
//    println!("Part 2 ans: {:?}", part2_ans);
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
            },
            Some(&'#') => {
                dir = (dir + 1) % dirs.len();
            },
            Some(_) => { unreachable!() },
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


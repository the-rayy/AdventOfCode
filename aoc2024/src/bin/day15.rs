use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day15.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);
    //
    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut input_split = input.split("\n\n");

    let mut grid: HashMap<(i32, i32), char> = input_split.next().unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let mut robot = grid.iter().filter(|(_, v)| **v == '@').next().unwrap().0.clone();
    grid.remove(&robot);

    for dir in input_split.next().unwrap().replace("\n", "").chars() {
        let dir = match dir {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let new_pos = (robot.0 + dir.0, robot.1 + dir.1);
        match grid.get(&new_pos) {
            None => robot = new_pos,
            Some('#') => (),
            Some('O') => {
                if shift(&mut grid, new_pos, dir) {
                    robot = new_pos;
                }
            },
            Some(_) => unreachable!(),
        }
    }

    grid.iter().filter(|(_, v)| **v == 'O').map(|(k, _)| (100 * k.0 + k.1) as u32).sum::<u32>()
}

fn shift(grid: &mut HashMap<(i32, i32), char>, start: (i32, i32), dir: (i32, i32)) -> bool {
    let mut new_pos = start;
    loop {
        new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
        match grid.get(&new_pos) {
            None => {break;},
            Some('#') => return false,
            Some(_) => {continue;}
        }
    }

    grid.insert(new_pos, 'O');
    grid.remove(&start);
    true
}


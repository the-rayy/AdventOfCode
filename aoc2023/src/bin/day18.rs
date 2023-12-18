use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day18.txt")
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
    let instructions = input.split("\n")
        .map(|line| {
            let mut splitted = line.split(" ");
            let dir = splitted.next().unwrap().chars().next().unwrap();
            let val = splitted.next().unwrap().parse::<u64>().unwrap();
            (dir, val)
        })
        .collect::<Vec<(char, u64)>>();

    let trench_lines = draw_trench_lines(instructions);

    shoelace(&trench_lines)
}


fn part2(input: &str) -> usize {
    let instructions = input.split("\n")
        .map(|line| {
            let line = line.split(" ").nth(2).unwrap().strip_prefix("(#").unwrap();
            let val = line.chars().take(5).collect::<String>();
            let val = u64::from_str_radix(&val, 16).unwrap();

            let dir = match line.chars().nth_back(1).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!()
            };
            (dir, val)
        })
        .collect::<Vec<(char, u64)>>();

    let grid = draw_trench_lines(instructions);

    shoelace(&grid)
}

fn draw_trench_lines(instructions: Vec<(char, u64)>) -> Vec<((i64, i64), (i64, i64))> {
    let mut grid = Vec::<((i64, i64), (i64, i64))>::new();

    let mut pos = (0, 0);

    for (dir, val) in instructions {
        let start = pos;
        let dir = match dir {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (1, 0),
            'D' => (-1, 0),
            _ => unreachable!()
        };
        let end = (pos.0 + dir.0 * val as i64, pos.1 + dir.1 * val as i64);
        grid.push((start, end));
        pos = end;
    }
    grid
}

//function that calculates the area of a simple polygon using shoelace formula
fn shoelace(grid: &Vec<((i64, i64), (i64, i64))>) -> usize {
    let mut area = 0.0;
    for ((x1, y1), (x2, y2)) in grid.iter().rev() {
        area += (x1 * y2 - x2 * y1) as f64;
    }
    let area = (area.abs() / 2.0) as usize;
    let perimeter = grid.iter().map(|((x1, y1), (x2, y2))| {
        ((x1 - x2).abs() + (y1 - y2).abs()) as usize
    }).sum::<usize>();
    area + perimeter / 2 + 1
}


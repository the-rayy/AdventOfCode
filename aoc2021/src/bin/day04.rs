use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day04.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let file = input.split("\n\n").collect::<Vec<&str>>();
    let numbers = file[0].split(",").map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut boards: Vec<Vec<i32>> = vec![];

    for i in 1..file.len() {
        let board: Vec<i32> = file[i].replace("\n", " ")
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .filter(|n| *n != -1)
            .collect();
        boards.push(board)
    }

    for number in numbers {
        for i in 0..boards.len() {
            boards[i] = mark(&boards[i], number);
            if bingo(&boards[i]) {
                return boards[i].iter().filter(|x| **x >= 0).sum::<i32>() * number
            }
        }
    }
    unreachable!()
}

fn part2(input: &str) -> i32 {
    let file = input.split("\n\n").collect::<Vec<&str>>();
    let numbers = file[0].split(",").map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut boards: Vec<Vec<i32>> = vec![];

    for i in 1..file.len() {
        let board: Vec<i32> = file[i].replace("\n", " ")
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .filter(|n| *n != -1)
            .collect();
        boards.push(board)
    }

    let mut last_score: i32 = 0;
    let mut win_boards: Vec<usize> = vec![];
    for number in numbers {
        for i in 0..boards.len() {
            if win_boards.contains(&i) {
                continue
            }
            boards[i] = mark(&boards[i], number);
            if bingo(&boards[i]) {
                win_boards.push(i);
                last_score = boards[i].iter().filter(|x| **x >= 0).sum::<i32>() * number;
            }
        }
    }
    last_score
}

fn mark(board: &Vec<i32>, num: i32) -> Vec<i32> {
    return board.iter()
        .map(|x| if *x == num {-1} else {*x})
        .collect()
}

fn bingo(board: &Vec<i32>) -> bool {
    for (x1, x2, x3, x4, x5) in board.iter().tuples() { //check rows
        if x1 + x2 + x3 + x4 + x5 == -5 {
            return true
        }
    };

    let b_size: i32 = 5;
    for x in 0..b_size {
        let idxs = vec![x, x+b_size, x+2*b_size, x+3*b_size, x+4*b_size];
        if board.iter().enumerate()
            .filter(|(i, _)| idxs.contains(&((*i) as i32)))
            .map(|(_, num)| *num)
            .sum::<i32>() == -5 {
            return true
        }
    }

    false
}

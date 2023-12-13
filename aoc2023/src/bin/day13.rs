use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day13.txt")
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
    input.split("\n\n")
        .map(|block| {
            block.split("\n")
                .map(|line| line.chars().collect::<Vec<char>>()).collect::<Block>()
        })
        .map(|block| score(&block))
        .sum()
}


fn part2(input: &str) -> usize {
    input.split("\n\n")
        .map(|block| {
            block.split("\n")
                .map(|line| line.chars().collect::<Vec<char>>()).collect::<Block>()
        })
        .map(|block| score(&block))
        .sum()
}

type Block = Vec<Vec<char>>;

fn score(block: &Block) -> usize {
    let row = match (0..block.len()-1).filter(|r| is_reflected_at(block, r+1)).next() {
        Some(x) => x+1,
        None => 0
    };

    let transposed_block = transpose(block);
    let column = match (0..transposed_block.len()-1).filter(|r| is_reflected_at(&transposed_block, r+1)).next() {
        Some(x) => x+1,
        None => 0
    };

    column + 100 * row
}

fn transpose(v: &Block) -> Block {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn is_reflected_at(b: &Block, i: usize) -> bool {
    let left = &b[..i];
    let right = &b[i..];

    left.iter().rev().zip(right)
        .all(|(l, r)| l == r)
}
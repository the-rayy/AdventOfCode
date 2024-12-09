use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day09.txt").expect("Unable to load input file");

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

#[derive(Debug)]
struct Filesystem {
    blocks: Vec<Option<u32>>,
}

impl Filesystem {
    fn from(input: &str) -> Self {
        let mut blocks = Vec::with_capacity(1000);
        let mut current_fid = 0;
        for (pos, c) in input.chars().enumerate() {
            if c == '\n' {continue};
            let c = c.to_digit(10).unwrap();
            match pos % 2 {
                0 => {
                    for _ in 0..c {blocks.push(Some(current_fid))};
                    current_fid += 1;
                }
                1 => {
                    for _ in 0..c {blocks.push(None)};
                }
                _ => unreachable!(),
            }
        };

        Self { blocks }
    }

    fn compress(&mut self) {
        let mut write_ptr = 0;
        let mut read_ptr = self.blocks.len() - 1;

        loop {
            if self.blocks[write_ptr].is_some() { write_ptr += 1; continue; }
            if self.blocks[read_ptr].is_none() { read_ptr -= 1; continue; }
            if write_ptr >= read_ptr { break; }

            self.blocks.swap(write_ptr, read_ptr);
        }
    }

    fn score(&self) -> u64 {
        self.blocks.iter().enumerate().map(|(pos, file_id)| {
            match file_id {
                Some(fid) => pos as u64 * (*fid as u64),
                None => 0,
            }
        }).sum()
    }
}

fn part1(input: &str) -> u64 {
    let mut fs = Filesystem::from(input);

    fs.compress();
    
    fs.score()
}
//
//fn part2(input: &str) -> u32 {
//    let mut right_counts: HashMap<i32, i32> = HashMap::with_capacity(1000);
//    let mut left = Vec::with_capacity(1000);
//
//    for line in input.lines() {
//        let mut split = line.split_whitespace();
//        let l = split.next().unwrap().parse::<i32>().unwrap();
//        let right = split.next().unwrap().parse::<i32>().unwrap();
//
//        *right_counts.entry(right).or_insert(0) += 1;
//
//        left.push(l);
//    }
//
//    left.iter()
//        .map(|l| right_counts.get(l).unwrap_or(&0) * l)
//        .sum::<i32>() as u32
//}

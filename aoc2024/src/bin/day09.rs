use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day09.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    let mut blocks = {
        let mut blocks = Vec::with_capacity(100_000);
        let mut current_fid = 0;
        for (pos, c) in input.chars().enumerate() {
            if c == '\n' {
                continue;
            };
            let c = c.to_digit(10).unwrap();
            match pos % 2 {
                0 => {
                    for _ in 0..c {
                        blocks.push(Some(current_fid))
                    }
                    current_fid += 1;
                }
                1 => {
                    for _ in 0..c {
                        blocks.push(None)
                    }
                }
                _ => unreachable!(),
            }
        }

        blocks
    };

    let mut write_ptr = 0;
    let mut read_ptr = blocks.len() - 1;

    loop {
        if blocks[write_ptr].is_some() {
            write_ptr += 1;
            continue;
        }
        if blocks[read_ptr].is_none() {
            read_ptr -= 1;
            continue;
        }
        if write_ptr >= read_ptr {
            break;
        }

        blocks.swap(write_ptr, read_ptr);
    }

    blocks
        .iter()
        .enumerate()
        .map(|(pos, file_id)| match file_id {
            Some(fid) => pos as u64 * (*fid as u64),
            None => 0,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut files = Vec::with_capacity(100_000);
    let mut freespace = Vec::with_capacity(100_000);

    let mut current_block = 0;
    for (pos, c) in input.chars().enumerate() {
        if c == '\n' {
            continue;
        };

        let c = c.to_digit(10).unwrap();

        if pos % 2 == 0 {
            files.push((current_block, current_block + c));
        } else {
            freespace.push((current_block, current_block + c));
        }

        current_block += c;
    }

    let max_fid = files.len();
    for fid in (0..max_fid).rev() {
        let file = &files[fid];
        let sid = freespace
            .iter()
            .position(|(start, end)| end - start >= file.1 - file.0 && *start <= file.0);

        match sid {
            Some(space_id) => {
                let (s_start, s_end) = freespace[space_id];
                let (f_start, f_end) = files[fid];

                files[fid] = (s_start, s_start + f_end - f_start);
                freespace[space_id] = (s_start + f_end - f_start, s_end);
            }
            None => {}
        }
    }

    files
        .iter()
        .enumerate()
        .map(|(fid, (start, end))| {
            (*start..*end)
                .map(|pos| pos as u64 * fid as u64)
                .sum::<u64>()
        })
        .sum()
}

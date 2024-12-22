use std::fs;
use std::time::Instant;

use std::ops::BitXor;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day22.txt").expect("Unable to load input file");

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
    let steps = 2000;
    input
        .lines()
        .map(|line| {
            let mut h = line.parse().unwrap();
            for _ in 0..steps {
                h = nxt(h);
            }
            h
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let steps = 2000;

    let hashmaps = input
        .lines()
        .map(|line| {
            let mut h = line.parse().unwrap();
            let last_digits = (0..steps + 1)
                .map(|_| {
                    let last_digit = h % 10;
                    h = nxt(h);
                    last_digit as i8
                })
                .collect::<Vec<_>>();

            let diffs = last_digits
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

            diffs
                .windows(4)
                .enumerate()
                .take(last_digits.len() - 4)
                .rev()
                .map(|(idx, seq)| {
                    (
                        seq.iter().cloned().collect::<Vec<_>>(),
                        last_digits[idx + 4] as u64,
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .fold(HashMap::new(), |mut acc, h| {
            for (k, v) in h {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        });

    *hashmaps.values().max().unwrap()
}

fn nxt(h: u64) -> u64 {
    let res = h * 64;
    let mix = h.bitxor(res);
    let h = mix % 16777216;

    let res = h / 32;
    let mix = h.bitxor(res);
    let h = mix % 16777216;

    let res = h * 2048;
    let mix = h.bitxor(res);
    let h = mix % 16777216;

    h
}

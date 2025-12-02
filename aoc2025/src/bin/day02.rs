use std::cmp::max;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day02.txt").expect("Unable to load input file");

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
    let mut c = 0;
    input.split(",").for_each(|line| {
        let mut s = line.trim().split("-");
        let start = s.next().unwrap();
        let mut half = {
            let (s, _) = split_in_half(start);
            s.parse::<u64>().unwrap()
        };
        let start = start.parse::<u64>().unwrap();
        let end = s.next().unwrap().parse::<u64>().unwrap();

        loop {
            let digits = half.checked_ilog10().unwrap() + 1;
            let foo = half * 10_u64.pow(digits) + half;

            if foo >= start && foo <= end {
                c += foo;
            }

            if foo >= end {
                break;
            }
            half += 1;
        }
    });

    c
}

fn part2(input: &str) -> u64 {
    let mut c = 0;
    let master = 11111111111111111_u64;
    input.split(",").for_each(|line| {
        let mut s = line.trim().split("-");
        let start = s.next().unwrap();
        let mid = max((start.len() as f64 / 2.0).floor() as usize, 1);
        let end = s.next().unwrap().parse::<u64>().unwrap();

        for i in 1..mid + 1 {
            let mut seed = {
                let (s, _) = start.split_at(i);
                s.parse::<u64>().unwrap()
            };
            let digits = i as u64;
            let seed = master % 10_u64.pow(digits);
            let start = start.parse::<u64>().unwrap();

            loop {
                let mut foo = seed;
                loop {
                    foo = foo * 10_u64.pow(digits) + seed;
                    if foo >= start && foo <= end {
                        c += foo;
                        dbg!(start, foo, end);
                    }

                    if foo >= end {
                        break;
                    }
                }
                seed += 1;
                let d = seed.checked_ilog10().unwrap() + 1;
                if d != digits { break; };
            }
        }
    });

    c
}

fn split_in_half(s: &str) -> (&str, &str) {
    let mid = max((s.len() as f64 / 2.0).floor() as usize, 1);
    s.split_at(mid)
}

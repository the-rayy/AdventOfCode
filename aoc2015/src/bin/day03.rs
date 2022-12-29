use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("data/day03.txt")
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
    input.chars()
        .map(|c| match c {
            '^' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, -1),
            _ => unreachable!()
        })
        .scan((0, 0), |acc, x| {
            *acc = (acc.0 + x.0, acc.1 + x.1);
            Some(*acc)
        })
        .collect::<HashSet<(i64, i64)>>()
        .len()
}

fn part2(input: &str) -> usize {
    input.chars()
        .map(|c| match c {
            '^' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, -1),
            _ => unreachable!()
        })
        .enumerate()
        .scan([(0, 0), (0, 0)], |acc, (i, x)| {
            if i % 2 == 0 {
                *acc = [acc[0], (acc[1].0 + x.0, acc[1].1 + x.1)]
            } else {
                *acc = [(acc[0].0 + x.0, acc[0].1 + x.1), acc[1]]
            }
            Some(*acc)
        })
        .flatten()
        .collect::<HashSet<(i64, i64)>>()
        .len()
}
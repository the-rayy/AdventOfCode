use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day01.txt")
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

fn part1(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.split("\n")
        .for_each(|line| {
            if line.is_empty() {
                return;
            }

            let mut split = line.split(" ");
            left.push(split.next().unwrap().parse::<i32>().unwrap());
            right.push(split.last().unwrap().parse::<i32>().unwrap());
        });

    left.sort();
    right.sort();

    left.iter().zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>() as u32
}

fn part2(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.split("\n")
        .for_each(|line| {
            if line.is_empty() {
                return;
            }

            let mut split = line.split(" ");
            left.push(split.next().unwrap().parse::<i32>().unwrap());
            right.push(split.last().unwrap().parse::<i32>().unwrap());
        });

    left.iter()
    .map(|l| {
            right.iter().filter(|r| l==*r).count() as i32 * l
        })
    .sum::<i32>() as u32
}


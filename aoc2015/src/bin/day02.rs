use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day02.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> i64 {
    input.split("\n")
        .map(|line| {
            line.split("x")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|d| [d[0]*d[1], d[1]*d[2], d[0]*d[2]])
        .map(|dim| 2 * (dim[0] + dim[1] + dim[2]) + dim.iter().min().unwrap())
        .sum()
}
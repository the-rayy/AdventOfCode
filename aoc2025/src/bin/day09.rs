use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day09.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    input.lines().map(|line| {
      let mut s = line.split(",");
      let x = s.next().unwrap().parse::<i64>().unwrap();
      let y = s.next().unwrap().parse::<i64>().unwrap();
      (x, y)
    }).combinations(2).map(|p| {
      ((p[0].0 - p[1].0).abs() + 1) * ((p[0].1 - p[1].1).abs() + 1)
}).min().unwrap() as u64
}


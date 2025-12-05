use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day05.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut s = input.split("\n\n");
    let ranges = s.next().unwrap().lines().map(|line| {
      let mut s = line.split("-");
      let start = s.next().unwrap().parse::<usize>().unwrap();
      let end = s.next().unwrap().parse::<usize>().unwrap();
      start..=end
    }).collect::<Vec<_>>();

    s.next().unwrap().lines().filter(|line| {
      let n = line.trim().parse::<usize>().unwrap();
      ranges.iter().any(|r| r.contains(&n))
    }).count() as u32
}


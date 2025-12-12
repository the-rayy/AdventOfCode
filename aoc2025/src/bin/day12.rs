use std::fs;
use std::time::Instant;

fn main() {
  let input = fs::read_to_string("data/day12.txt").expect("Unable to load input file");

  let part1_start = Instant::now();
  let part1_ans = part1(&input);
  println!("Part 1 time: {:.2?}", part1_start.elapsed());
  println!("Part 1 ans: {:?}", part1_ans);
}

fn part1(input: &str) -> u64 {
  input
    .lines()
    .skip(30)
    .filter(|input| {
      let dimx = &input[0..2].parse::<usize>().unwrap();
      let dimy = &input[3..5].parse::<usize>().unwrap();
      let s0 = &input[7..9].parse::<usize>().unwrap();
      let s1 = &input[10..12].parse::<usize>().unwrap();
      let s2 = &input[13..15].parse::<usize>().unwrap();
      let s3 = &input[16..18].parse::<usize>().unwrap();
      let s4 = &input[19..21].parse::<usize>().unwrap();
      let s5 = &input[22..24].parse::<usize>().unwrap();

      let req_area = (s0 + s1 + s2 + s3 + s4 + s5) * 9;
      let area = dimx * dimy;

      area >= req_area
    })
    .count() as u64
}

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
    .split("\n\n")
    .last()
    .unwrap()
    .lines()
    .filter(|input| {
      let dimx = &input[0..2].parse::<usize>().unwrap();
      let dimy = &input[3..5].parse::<usize>().unwrap();
      let req_area = &input[7..].split(" ").map(|x| x.parse::<usize>().unwrap() * 9).sum::<usize>();

      let area = dimx * dimy;

      area >= req_area
    })
    .count() as u64
}

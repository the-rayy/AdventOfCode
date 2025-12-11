use std::fs;
use std::time::Instant;

use hashbrown::HashMap;
use itertools::Itertools;

fn main() {
  let input = fs::read_to_string("data/day11.txt").expect("Unable to load input file");

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
  let graph = input
    .lines()
    .map(|line| {
      let mut foo = line.split(" ");
      let input = foo.next().unwrap().strip_suffix(":").unwrap();
      let outputs = foo.collect_vec();
      (input, outputs)
    })
    .collect::<HashMap<_, _>>();

  let start = "you";
  dfs(&graph, start)
}

fn dfs(graph: &HashMap<&str, Vec<&str>>, current: &str) -> u64 {
  if current == "out" {
    return 1;
  }

  graph[current].iter().map(|x| dfs(graph, x)).sum::<u64>()
}

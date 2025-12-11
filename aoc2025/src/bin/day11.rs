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

  let part2_start = Instant::now();
  let part2_ans = part2(&input);
  println!("Part 2 time: {:.2?}", part2_start.elapsed());
  println!("Part 2 ans: {:?}", part2_ans);
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

  dfs(&graph, "you", "out", &vec![], &mut HashMap::new())
}

fn part2(input: &str) -> u64 {
  let graph = input
    .lines()
    .map(|line| {
      let mut foo = line.split(" ");
      let input = foo.next().unwrap().strip_suffix(":").unwrap();
      let outputs = foo.collect_vec();
      (input, outputs)
    })
    .collect::<HashMap<_, _>>();

  dfs(
    &graph,
    "svr",
    "fft",
    &vec!["dac", "out"],
    &mut HashMap::new(),
  ) * dfs(
    &graph,
    "fft",
    "dac",
    &vec!["svr", "out"],
    &mut HashMap::new(),
  ) * dfs(
    &graph,
    "dac",
    "out",
    &vec!["svr", "fft"],
    &mut HashMap::new(),
  ) + dfs(
    &graph,
    "svr",
    "dac",
    &vec!["fft", "out"],
    &mut HashMap::new(),
  ) * dfs(
    &graph,
    "dac",
    "fft",
    &vec!["svr", "out"],
    &mut HashMap::new(),
  ) * dfs(
    &graph,
    "fft",
    "out",
    &vec!["svr", "dac"],
    &mut HashMap::new(),
  )
}

fn dfs<'a>(
  graph: &HashMap<&str, Vec<&'a str>>,
  current: &'a str,
  target: &str,
  forbidden: &[&str],
  cache: &mut HashMap<&'a str, u64>,
) -> u64 {
  if let Some(x) = cache.get(current) {
    return *x;
  }

  if current == target {
    cache.insert(current, 1);
    return 1;
  }

  if forbidden.contains(&current) {
    cache.insert(current, 0);
    return 0;
  }

  let x = graph[current]
    .iter()
    .map(|x| dfs(graph, x, target, forbidden, cache))
    .sum::<u64>();
  cache.insert(current, x);

  x
}

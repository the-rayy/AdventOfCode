use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day23.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
  let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
  for line in input.lines() {
    let node1 = &line[0..2];
    let node2 = &line[3..5];

    graph.entry(node1).or_insert(HashSet::new()).insert(node2);
    graph.entry(node2).or_insert(HashSet::new()).insert(node1);
  }

  let mut triplets = HashSet::new();
  graph.keys().filter(|x| x.starts_with("t")).for_each(|level0| {
    let level1 = graph.get(level0).unwrap();
    for llevel1 in level1 {
      let level2 = graph.get(llevel1).unwrap();
      let intersection = level1.intersection(level2);

      for iintersection in intersection {
        let mut triplet = [level0, llevel1, iintersection];
        triplet.sort();
        triplets.insert(triplet);
      }
    }
});

  triplets.len() as u32
}

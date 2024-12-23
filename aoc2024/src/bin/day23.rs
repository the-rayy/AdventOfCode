use std::time::Instant;
use std::{collections::VecDeque, fs};

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day23.txt").expect("Unable to load input file");

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
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let node1 = &line[0..2];
        let node2 = &line[3..5];

        graph.entry(node1).or_insert(HashSet::new()).insert(node2);
        graph.entry(node2).or_insert(HashSet::new()).insert(node1);
    }

    let mut triplets = HashSet::new();
    graph
        .keys()
        .filter(|x| x.starts_with("t"))
        .for_each(|level0| {
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

fn part2(input: &str) -> String {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let node1 = &line[0..2];
        let node2 = &line[3..5];

        graph.entry(node1).or_insert(HashSet::new()).insert(node2);
        graph.entry(node2).or_insert(HashSet::new()).insert(node1);
    }

    let lans = input
        .lines()
        .map(|line| {
            let node1 = &line[0..2];
            let node2 = &line[3..5];

            find_lan(node1, node2, &graph)
        })
        .collect::<Vec<_>>();

    let biggest_lan = lans.into_iter().max_by_key(|x| x.len()).unwrap();
    let mut biggest_lan = biggest_lan.into_iter().collect::<Vec<_>>();
    biggest_lan.sort();
    biggest_lan.join(",").to_string()
}

fn find_lan<'a>(
    comp1: &'a str,
    comp2: &'a str,
    graph: &'a HashMap<&str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut lan = HashSet::new();
    lan.insert(comp1);
    let mut candidates = VecDeque::new();
    let mut visited = HashSet::new();
    candidates.push_back(comp2);

    while let Some(candidate) = candidates.pop_front() {
        if visited.contains(candidate) {
            continue;
        }

        visited.insert(candidate);

        let neighbors = graph.get(candidate).unwrap();

        if lan.intersection(neighbors).count() == lan.len() {
            lan.insert(candidate);
            candidates.extend(neighbors);
        }
    }

    lan
}

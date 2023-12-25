use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::process::Command;
use std::time::Instant;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day25.txt")
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


fn part1(input: &str) -> usize {
    let edges = input.split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let source = parts.next().unwrap();
            let targets = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
            (source, targets)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let reverse_edges: HashMap<&str, Vec<&str>> = edges.iter()
        .flat_map(|(source, targets)| {
            targets.iter().map(move |target| (*target, *source))
        })
        .into_group_map();

    let mut sample_size = 100_usize;

    loop {
        let mut counts = HashMap::<(&str, &str), usize>::new();
        (0..sample_size).into_par_iter()
            .map(|_| {
                let k1 = *edges.keys().chain(reverse_edges.keys()).choose(&mut rand::thread_rng()).unwrap();
                let k2 = *edges.keys().chain(reverse_edges.keys()).choose(&mut rand::thread_rng()).unwrap();
                path(&edges, &reverse_edges, k1, k2)
            })
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|path| {
                *counts.entry(*path).or_insert(0) += 1;
            });

        //get top 6 counts
        let mut counts = counts.iter().collect::<Vec<_>>();
        counts.sort_by_key(|(_, &count)| count);
        counts.reverse();

        let banlist = counts[0..3].iter()
            .map(|(&(n1, n2), _)| (n1, n2))
            .collect::<Vec<(&str, &str)>>();

        let s0 = subgraph_size(&edges, &reverse_edges, banlist[0].0, &vec![]);
        let s1 = subgraph_size(&edges, &reverse_edges, banlist[0].0, &banlist);
        let s2 = subgraph_size(&edges, &reverse_edges, banlist[0].1, &banlist);

        if s1 == s0 || s2 == s0 {
            println!("Found it! But not working :(");
            sample_size *= 2;
            continue;
        }

        return s1 * s2
    }
}

fn path<'a>(edges: &'a HashMap<&'a str, Vec<&'a str>>, reverse_edges: &'a HashMap<&'a str, Vec<&'a str>>, source: &'a str, target: &'a str) -> Vec<(&'a str, &'a str)> {
    let mut queue = VecDeque::new();
    queue.push_back((source, vec![]));
    let mut visited = vec![];
    while let Some((node, path)) = queue.pop_front() {
        if node == target && path.len() == 1 {
            continue;
        }
        if node == target {
            return path;
        }

        visited.push(node);
        for &next in edges.get(node).unwrap_or(&vec![]).iter().chain(reverse_edges.get(node).unwrap_or(&vec![])) {
            if !visited.contains(&next) {
                let mut new_path = path.clone();
                let path_part = if node > next {
                    (next, node)
                } else {
                    (node, next)
                };
                new_path.push(path_part);
                queue.push_back((next, new_path));
            }
        }
    }

    vec![]
}

fn subgraph_size(edges: &HashMap<&str, Vec<&str>>, reverse_edges: &HashMap<&str, Vec<&str>>, node: &str, banlist: &Vec<(&str, &str)>) -> usize {
    let mut queue = vec![node];
    let mut visited = HashSet::<&str>::new();
    while let Some(node) = queue.pop() {
        visited.insert(node);
        for &next in edges.get(node).unwrap_or(&vec![]).iter().chain(reverse_edges.get(node).unwrap_or(&vec![])) {
            if !visited.contains(&next) && !banlist.iter().any(|(n1, n2)| {
                (node == *n1 && next == *n2) || (node == *n2 && next == *n1)
            }) {
                queue.push(next);
            }
        }
    }

    visited.len()
}

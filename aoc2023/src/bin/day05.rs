use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::path::Component::ParentDir;

fn main() {
    let input = fs::read_to_string("data/day05.txt")
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

fn part1(input: &str) -> u32 {
    let (seeds, mappings) = parse(input);

    seeds.iter()
        .map(|&s| {
            let s = mappings.get("seed-to-soil").unwrap().map(s);
            let s = mappings.get("soil-to-fertilizer").unwrap().map(s);
            let s = mappings.get("fertilizer-to-water").unwrap().map(s);
            let s = mappings.get("water-to-light").unwrap().map(s);
            let s = mappings.get("light-to-temperature").unwrap().map(s);
            let s = mappings.get("temperature-to-humidity").unwrap().map(s);
            let s = mappings.get("humidity-to-location").unwrap().map(s);
            s
        }).min().unwrap()
}

fn parse(input: &str) -> (Vec<u32>, HashMap<&str, Mapping>){
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let maps = blocks.map(parse_block)
        .collect::<HashMap<&str, Mapping>>();

    (seeds, maps)
}

fn parse_block(block: &str) -> (&str, Mapping) {
    let mut lines = block.split("\n");
    let name = lines.next().unwrap().strip_suffix(" map:").unwrap();
    let mapping = Mapping{
        rows: lines.map(|line| {
            let mut splitted = line.split(" ");
            MappingRow{
                destination: splitted.next().unwrap().parse::<u32>().unwrap(),
                source: splitted.next().unwrap().parse::<u32>().unwrap(),
                range: splitted.next().unwrap().parse::<u32>().unwrap(),
            }
        })
            .collect(),
    };

    (name, mapping)
}

#[derive(Debug)]
struct Mapping {
    rows: Vec<MappingRow>
}

impl Mapping {
    fn map(&self, x: u32) -> u32 {
        for row in &self.rows {
            match row.map(x) {
                Some(y) => { return y; }
                None => { continue; }
            }
        }

        x
    }
}

#[derive(Debug)]
struct MappingRow {
    source: u32,
    destination: u32,
    range: u32,
}

impl MappingRow {
    fn map(&self, x: u32) -> Option<u32> {
        if x >= self.source && x < self.source + self.range {
            Some(x - self.source + self.destination)
        } else { None }
    }
}

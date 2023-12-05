use std::cmp::max;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day05.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

const MAPPING_KEYS: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location"
];

fn part1(input: &str) -> u32 {
    let (seeds, mappings) = parse(input);

    seeds.iter()
        .map(|&s| {
            let mut s = s.clone();
            for key in MAPPING_KEYS {
                s = mappings.get(key).unwrap().map(s);
            }
            s
        }).min().unwrap()
}

fn part2(input: &str) -> u32 {
    let (seeds, mappings) = parse(input);

    let mut seeds = seeds.iter()
        .tuples()
        .map(|(&start, &len)| {
            HashSet::from([start, start + len - 1])
        })
        .collect::<Vec<HashSet<u32>>>();

    println!("{:?}", seeds);

    for key in MAPPING_KEYS {
        seeds = seeds.iter()
            .map(|seed| {
                let m = mappings.get(key).unwrap();
                let min = *seed.iter().min().unwrap();
                let max = *seed.iter().max().unwrap();
                println!("key {} disc: {:?}", key, m.discontinuities());
                m.discontinuities().union(seed).into_iter()
                    .filter(|&&x| x >= min && x <= max)
                    .map(|&x| m.map(x))
                    // .map(|&x| x)
                    .collect::<HashSet<u32>>()
            })
            .collect::<Vec<HashSet<u32>>>();

        println!("after {}: {:?}", key, seeds);
    }

    *seeds.iter().flatten().min().unwrap()
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

    fn discontinuities(&self) -> HashSet<u32> {
        self.rows.iter()
            .map(|r| {
                [max(r.start() as i32 - 1, 0) as u32,
                    r.start(),
                    r.end(),
                    r.end()+1
                ]
            })
            .flatten()
            .collect()
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

    fn start(&self) -> u32 {
        self.source
    }

    fn end(&self) -> u32 {
        self.source + self.range
    }
}

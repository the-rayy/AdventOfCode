use std::fs;
use std::time::Instant;
use std::collections::HashMap;
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

fn part1(input: &str) -> u64 {
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

fn part2(input: &str) -> u64 {
    let (seeds, mappings) = parse(input);

    let mut seeds = seeds.iter()
        .tuples()
        .map(|(&start, &len)| {
            Range{start, end: start + len - 1}
        })
        .collect::<Vec<Range>>();

    for key in MAPPING_KEYS {
        seeds = seeds.iter()
            .map(|seed| {
                let m = mappings.get(key).unwrap();
                let foo = m.rows.iter().map(|r| r.as_range()).collect::<Vec<Range>>();
                let new_seeds = seed.split(&foo);
                new_seeds.iter()
                    .map(|r| {
                        Range{start: m.map(r.start), end: m.map(r.end)}
                    })
                    .collect::<Vec<Range>>()
            })
            .flatten()
            .collect()
    }

    seeds.iter()
        .map(|r| {r.start})
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Vec<u64>, HashMap<&str, Mapping>){
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

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
                destination: splitted.next().unwrap().parse::<u64>().unwrap(),
                source: splitted.next().unwrap().parse::<u64>().unwrap(),
                range: splitted.next().unwrap().parse::<u64>().unwrap(),
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
    fn map(&self, x: u64) -> u64 {
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
    source: u64,
    destination: u64,
    range: u64,
}

impl MappingRow {
    fn map(&self, x: u64) -> Option<u64> {
        if x >= self.source && x < self.source + self.range {
            Some(x - self.source + self.destination)
        } else { None }
    }

    fn as_range(&self) -> Range {
        Range{start: self.source, end: self.source + self.range}
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn split(&self, others: &Vec<Range>) -> Vec<Range> {
        let mut splitpoints = Vec::<u64>::new();
        splitpoints.push(self.start);
        splitpoints.push(self.end);
        for o in others {
            if o.start > 0 {
                splitpoints.push(o.start - 1);
            }
            splitpoints.push(o.start);
            splitpoints.push(o.end);
            splitpoints.push(o.end + 1);
        }
        splitpoints = splitpoints.into_iter().filter(|x| *x >= self.start && *x <= self.end).collect();
        splitpoints.sort();

        splitpoints.into_iter()
            .tuples()
            .map(|(start, end)| Range{start, end })
            .collect()
    }
}

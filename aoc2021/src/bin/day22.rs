use std::cmp::{max, min};
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day22.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone)]
struct RebootStep {
    include: bool,
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
}

#[derive(Debug)]
struct Cube {
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
    cuts: Vec<Cube>,
}

impl Cube {
    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let x = intersection(self.x, other.x);
        let y = intersection(self.y, other.y);
        let z = intersection(self.z, other.z);
        if x.is_none() || y.is_none() || z.is_none() {
            return None;
        };
        return Some(Cube {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            cuts: vec![],
        });
    }

    fn subtract(&mut self, other: &Cube) {
        let intersection = self.intersect(other);
        if intersection.is_none() {
            return;
        }
        for cut in self.cuts.iter_mut() {
            cut.subtract(other);
        }
        self.cuts.push(intersection.unwrap());
    }

    fn vol(&self) -> usize {
        (self.x[1] - self.x[0] + 1) as usize *
            (self.y[1] - self.y[0] + 1) as usize *
            (self.z[1] - self.z[0] + 1) as usize -
            self.cuts.iter().map(|c| c.vol()).sum::<usize>()
    }
}

fn intersection(a: [i64; 2], b: [i64; 2]) -> Option<[i64; 2]> {
    let ret = [
        max(a[0], b[0]),
        min(a[1], b[1])
    ];
    if ret[1] >= ret[0] {
        Some(ret)
    } else {
        None
    }
}

fn part1(input: &str) -> i64 {
    let reboot_steps = load(input);
    let reboot_steps: Vec<RebootStep> = reboot_steps.iter()
        .filter(|step| step.x[0].abs() <= 50)
        .map(|step| *step)
        .collect();

    simulate(reboot_steps)
}

fn part2(input: &str) -> i64 {
    let reboot_steps = load(input);
    simulate(reboot_steps)
}


fn load(input: &str) -> Vec<RebootStep> {
    let mut reboot_steps: Vec<RebootStep> = Vec::new();
    let rg = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    for line in input.split("\n") {
        let captures = rg.captures(line).unwrap();
        let step = RebootStep {
            include: captures[1] == *"on",
            x: [
                captures[2].parse::<i64>().unwrap(),
                captures[3].parse::<i64>().unwrap(),
            ],
            y: [
                captures[4].parse::<i64>().unwrap(),
                captures[5].parse::<i64>().unwrap(),
            ],
            z: [
                captures[6].parse::<i64>().unwrap(),
                captures[7].parse::<i64>().unwrap(),
            ],
        };
        reboot_steps.push(step);
    }
    reboot_steps
}

fn simulate(reboot_steps: Vec<RebootStep>) -> i64 {
    let mut cubes: Vec<Cube> = Vec::new();

    for step in reboot_steps {
        let cube = Cube {
            x: step.x,
            y: step.y,
            z: step.z,
            cuts: Vec::new()
        };
        for c in cubes.iter_mut() {
            c.subtract(&cube)
        }
        if step.include {
            cubes.push(cube);
        }
    }

    cubes.iter().map(|c| c.vol()).sum::<usize>() as i64
}
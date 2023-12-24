use std::fs;
use std::process::Command;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day24.txt")
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


fn part1(input: &str) -> usize {
    let loweer_bound = 200000000000000.0;
    let upper_bound = 400000000000000.0;
    let hails = input.split("\n")
        .map(|l| Hail::new(l))
        .collect::<Vec<_>>();

    hails.iter()
        .combinations(2)
        .map(|c| {
            let line1 = c[0].line_params();
            let line2 = c[1].line_params();
            (c[0], c[1], crossing_point(line1, line2))
        })
        .filter(|(hail, hail2, p)| {
            p.is_some()
            && p.unwrap().0 >= loweer_bound && p.unwrap().0 <= upper_bound
            && p.unwrap().1 >= loweer_bound && p.unwrap().1 <= upper_bound
            && hail.is_in_future(p.unwrap())
            && hail2.is_in_future(p.unwrap())
        })
        .count()
}

fn part2(_: &str) -> usize {
    let output = Command::new("python3")
        .args(["src/bin/day24_2.py"])
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap().trim().parse::<usize>().unwrap()
}

fn crossing_point(line1: (f64, f64), line2: (f64, f64)) -> Option<(f64, f64)> {
    if line1.0 == line2.0 {
        return None;
    }

    let x = (line2.1 - line1.1) / (line1.0 - line2.0);
    let y = line1.0 * x + line1.1;

    Some((x, y))
}

#[derive(Debug)]
struct Hail {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

impl Hail {
    fn new(input: &str) -> Hail {
        let input = input.replace(" @", ",");
        let mut splitted = input.split(", ");
        let pos = (
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
        );
        let vel = (
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
           splitted.next().unwrap().trim().parse::<f64>().unwrap(),
        );
        Hail{pos, vel}
    }

    fn line_params(&self) -> (f64, f64) {
        let pos2 = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1, self.pos.2 + self.vel.2);

        let a = (self.pos.1 - pos2.1) / (self.pos.0 - pos2.0);
        let b = self.pos.1 - a * self.pos.0;

        (a, b)
    }

    fn signums(&self) -> (f64, f64, f64) {
        (self.vel.0.signum(), self.vel.1.signum(), self.vel.2.signum())
    }

    fn is_in_future(&self, p: (f64, f64)) -> bool {
        let x = (p.0 - self.pos.0) / self.vel.0;
        let y = (p.1 - self.pos.1) / self.vel.1;
        // let z = (p.2 - self.pos.2) / self.vel.2;

        // x == y && y == z && x > 0.0
        x > 0.0 && y > 0.0
    }
}
use std::fs;
use std::time::Instant;

use hashbrown::HashSet;

fn main() {
  let input = fs::read_to_string("data/day12.txt").expect("Unable to load input file");

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
  let s = input.split("\n\n");
  let shapes = s.clone().take(6).map(Shape::from_lines).collect::<Vec<_>>();
  let regions = s
    .last()
    .unwrap()
    .lines()
    .map(Region::from_line)
    .collect::<Vec<_>>();

  let foo = regions.iter().map(|r| {
    let required_area = r.shapes.iter().enumerate().map(|(i, c)| shapes[i].area * c).sum::<usize>();
    let ideal_area = r.shapes.iter().map(|c| c*9).sum::<usize>();
    if r.area >= ideal_area {
      1
    } else 
    if required_area > r.area {
      -1
    } else { 0 }
  }).collect::<Vec<_>>();

  println!("can do: {}", foo.iter().filter(|i| **i == 1).count());
  println!("cant do: {}", foo.iter().filter(|i| **i == -1).count());
  println!("dunno: {}", foo.iter().filter(|i| **i == 0).count());

  0
}

#[derive(Debug)]
struct Shape {
  area: usize,
  variants: Vec<Vec<(usize, usize)>>,
}

impl Shape {
  fn from_lines(input: &str) -> Self {
    let s0 = input
      .lines()
      .skip(1)
      .enumerate()
      .map(|(i, l)| {
        l.chars()
          .enumerate()
          .filter(|(_, c)| *c == '#')
          .map(move |(j, _)| (i, j))
      })
      .flatten()
      .collect::<Vec<_>>();

    let rot = |foo: (usize, usize)| (2 - foo.1, foo.0);
    let flip = |foo: (usize, usize)| (2 - foo.0, foo.1);

    let s1 = s0.iter().cloned().map(rot).collect::<Vec<_>>();
    let s2 = s1.iter().cloned().map(rot).collect::<Vec<_>>();
    let s3 = s2.iter().cloned().map(rot).collect::<Vec<_>>();

    let r0 = s0.iter().cloned().map(flip).collect::<Vec<_>>();
    let r1 = s1.iter().cloned().map(flip).collect::<Vec<_>>();
    let r2 = s2.iter().cloned().map(flip).collect::<Vec<_>>();
    let r3 = s3.iter().cloned().map(flip).collect::<Vec<_>>();

    let area = s0.len();

    let variants = vec![s0, s1, s2, s3, r0, r1, r2, r3];
    let variants = {
      let mut seen = HashSet::new();
      let mut result = Vec::new();

      for mut t in variants {
        t.sort();
        if seen.insert(t.clone()) {
          result.push(t);
        }
      }
      result
    };

    Self { area: area, variants: variants }
  }
}

#[derive(Debug)]
struct Region {
  dims: (usize, usize),
  shapes: Vec<usize>,
  area: usize,
}

impl Region {
  fn from_line(input: &str) -> Self {
    let input = input.replace("x", " ").replace(":", "");
    let mut s = input.split(" ");
    let dimx = s.next().unwrap().parse::<usize>().unwrap();
    let dimy = s.next().unwrap().parse::<usize>().unwrap();
    let shapes = s.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let area = dimx * dimy;

    Self {
      dims: (dimx, dimy),
      shapes: shapes,
      area: area,
    }
  }
}

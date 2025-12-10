use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;
use microlp::{LinearExpr, Problem};
use rayon::prelude::*;

fn main() {
  let input = fs::read_to_string("data/day10.txt").expect("Unable to load input file");

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
  input
    .lines()
    .map(|line| {
      let line = line.split(" ").collect::<Vec<&str>>();
      let target = line[0]
        .chars()
        .filter_map(|c| match c {
          '[' | ']' => None,
          '.' => Some(0),
          '#' => Some(1),
          _ => None,
        })
        .collect_vec();
      let buttons = line
        .iter()
        .skip(1)
        .rev()
        .skip(1)
        .map(|btns| {
          btns
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec()
        })
        .collect_vec();

      let lights = vec![0; target.len()];

      let mut n = 1_u64;
      loop {
        for comb in buttons.iter().combinations_with_replacement(n as usize) {
          let mut lights = lights.clone();
          for btn in comb {
            lights = press(&lights, btn, 1);
          }
          if finished(&lights, &target) {
            return n;
          }
        }
        n += 1;
      }
    })
    .sum()
}

fn part2(input: &str) -> u64 {
  input
    .par_lines()
    .map(|line| {
      let line = line.split(" ").collect::<Vec<&str>>();
      let target = line[line.len() - 1]
        .replace("{", "")
        .replace("}", "")
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

      let buttons = line
        .iter()
        .skip(1)
        .rev()
        .skip(1)
        .map(|btns| {
          btns
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec()
        })
        .sorted_by_key(|b| b.len())
        .rev()
        .collect_vec();

      let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);
      let max = target.iter().max().unwrap();
      let variables = (0..buttons.len())
        .map(|_| problem.add_integer_var(1.0, (0, *max as i32)))
        .collect_vec();

      for (i, t) in target.iter().enumerate() {
        problem.add_constraint(
          buttons
            .iter()
            .zip(&variables)
            .filter(|(b, _)| b.contains(&i))
            .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
              ex.add(var, 1.0);
              ex
            }),
          microlp::ComparisonOp::Eq,
          *t as f64,
        );
      }
      problem.solve().unwrap().objective().round() as u64
    })
    .sum()
}

fn press(lights: &Vec<u64>, button: &Vec<usize>, times: u64) -> Vec<u64> {
  let mut lights = lights.clone();

  for btn in button {
    lights[*btn] += times;
  }

  lights
}

fn finished(lights: &Vec<u64>, target: &Vec<u64>) -> bool {
  lights.iter().zip(target).all(|(l, t)| l % 2 == *t)
}

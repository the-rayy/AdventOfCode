use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;
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
      depress(&buttons, &target, 0).unwrap()
    })
    .sum()
}

fn depress(buttons: &[Vec<usize>], lights: &Vec<i64>, presses: u64) -> Option<u64> {
  if lights.iter().any(|x| *x < 0) {
    return None;
  }

  if buttons.is_empty() && lights.iter().all(|x| *x == 0) {
    return Some(presses);
  }

  if buttons.is_empty() {
    return None;
  }

  let lights_needed_to_light = lights
    .iter()
    .enumerate()
    .filter(|(_, x)| **x > 0)
    .map(|(i, _)| i)
    .collect::<Vec<usize>>();

  for i in &lights_needed_to_light {
    let possible_buttons = buttons
      .iter()
      .enumerate()
      .filter(|(_, b)| b.contains(&i))
      .map(|(i, _)| i)
      .collect_vec();
    if possible_buttons.len() == 0 {
      return None;
    }
    if possible_buttons.len() == 1 {
      let button = &buttons[possible_buttons[0]];
      let mut tail = buttons.iter().cloned().collect_vec();
      tail.remove(possible_buttons[0]);

      let press = lights[*i].abs() as u64;
      let lights = press2(lights, button, press);
      return depress(&tail, &lights, presses + press)
    }
  }

  let button = &buttons[0];
  let tail = &buttons[1..];

  let max = lights
    .iter()
    .enumerate()
    .filter(|(i, _)| button.contains(i))
    .map(|(_, x)| x)
    .min()
    .unwrap();
  for i in (0..=*max as u64).rev() {
    let lights = press2(lights, button, i);
    match depress(tail, &lights, presses + i) {
      None => continue,
      Some(x) => {
        return Some(x);
      }
    }
  }

  None
}

fn press(lights: &Vec<u64>, button: &Vec<usize>, times: u64) -> Vec<u64> {
  let mut lights = lights.clone();

  for btn in button {
    lights[*btn] += times;
  }

  lights
}

fn press2(lights: &Vec<i64>, button: &Vec<usize>, times: u64) -> Vec<i64> {
  let mut lights = lights.clone();

  for btn in button {
    lights[*btn] -= times as i64;
  }

  lights
}

fn finished(lights: &Vec<u64>, target: &Vec<u64>) -> bool {
  lights.iter().zip(target).all(|(l, t)| l % 2 == *t)
}

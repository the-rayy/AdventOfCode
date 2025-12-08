use std::fs;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day08.txt").expect("Unable to load input file");

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
    let foo = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64, i64)>()
        })
        .combinations(2)
        .map(|comb| {
            let d = (comb[0].unwrap().0 - comb[1].unwrap().0).pow(2)
                + (comb[0].unwrap().1 - comb[1].unwrap().1).pow(2)
                + (comb[0].unwrap().2 - comb[1].unwrap().2).pow(2);
            (comb[0].unwrap(), comb[1].unwrap(), d)
        })
        .sorted_by_key(|(_, _, d)| d.clone())
        .take(1000)
        .collect::<Vec<_>>();

    let mut circuits: Vec<Vec<(i64, i64, i64)>> = vec![];

    for (first, second, _) in foo {
      let cloned = circuits.clone();
      let first_idx = cloned.iter().find_position(|p| p.contains(&first));
      let second_idx = cloned.iter().find_position(|p| p.contains(&second));

      match (first_idx, second_idx) {
        (None, None) => {
          circuits.push(vec![first.clone(), second.clone()]);
        }
        (Some(x), Some(y)) => {
          if x == y { continue; };
          let second = circuits.get(y.0).unwrap().clone();
          circuits[x.0].extend(second.iter().cloned());
          circuits.remove(y.0);
        },
        (Some(x), None) => {
          circuits[x.0].push(second);
        },
        (None, Some(y)) => {
          circuits[y.0].push(first);
        }
      }
    }

    circuits.iter().map(|x| x.len()).sorted().rev().take(3).product::<usize>() as u64
}

fn part2(input: &str) -> u64 {
    let foo = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64, i64)>()
        })
        .combinations(2)
        .map(|comb| {
            let d = (comb[0].unwrap().0 - comb[1].unwrap().0).pow(2)
                + (comb[0].unwrap().1 - comb[1].unwrap().1).pow(2)
                + (comb[0].unwrap().2 - comb[1].unwrap().2).pow(2);
            (comb[0].unwrap(), comb[1].unwrap(), d)
        })
        .sorted_by_key(|(_, _, d)| d.clone())
        .collect::<Vec<_>>();

    let mut circuits: Vec<Vec<(i64, i64, i64)>> = vec![];

    for (first, second, _) in foo {
      let cloned = circuits.clone();
      let first_idx = cloned.iter().find_position(|p| p.contains(&first));
      let second_idx = cloned.iter().find_position(|p| p.contains(&second));

      match (first_idx, second_idx) {
        (None, None) => {
          circuits.push(vec![first.clone(), second.clone()]);
        }
        (Some(x), Some(y)) => {
          if x == y { continue; };
          let second = circuits.get(y.0).unwrap().clone();
          circuits[x.0].extend(second.iter().cloned());
          circuits.remove(y.0);
        },
        (Some(x), None) => {
          circuits[x.0].push(second);
        },
        (None, Some(y)) => {
          circuits[y.0].push(first);
        }
      }

      if circuits.len() == 1 && circuits[0].len() == input.lines().count() {
        return (first.0 * second.0) as u64
      }
    }
  unreachable!();
}

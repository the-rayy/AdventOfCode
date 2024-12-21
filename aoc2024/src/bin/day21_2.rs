use std::fs;
use std::time::Instant;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
  
fn main() {
    let input = fs::read_to_string("data/day21.txt").expect("Unable to load input file");

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
  input.lines().map(|l| {
    let len = solve(l, 2);
    let numeric = l.strip_suffix("A").unwrap().parse::<u64>().unwrap();
    len * numeric
  }).sum()
}

fn part2(input: &str) -> u64 {
  input.lines().map(|l| {
    let len = solve(l, 25);
    let numeric = l.strip_suffix("A").unwrap().parse::<u64>().unwrap();
    len * numeric
  }).sum()
}

fn solve(input: &str, robots: u32) -> u64 {
  let numpad_paths = numpad(input);
  numpad_paths.iter().map(|p| {
    let p = vec!['A'].into_iter().chain(p.iter().cloned()).collect::<Vec<char>>();
    let mut cache = HashMap::new();
    p.windows(2).map(|c| arrowpad(c[0], c[1], robots-1, &mut cache)).sum()
  }).min().unwrap()
}

fn numpad_position(c: char) -> (i32, i32) {
  match c {
    '7' => (0, 0),
    '8' => (1, 0),
    '9' => (2, 0),
    '4' => (0, 1),
    '5' => (1, 1),
    '6' => (2, 1),
    '1' => (0, 2),
    '2' => (1, 2),
    '3' => (2, 2),
    '0' => (1, 3),
    'A' => (2, 3),
    _ => panic!("Invalid position: {}", c),
  }
}

fn numpad(chars: &str) -> HashSet<Vec<char>> {
  let mut pos = 'A';
  let mut moves = vec![Vec::new()];

  for c in chars.chars() {
    let dir = (numpad_position(c).0 - numpad_position(pos).0, numpad_position(c).1 - numpad_position(pos).1);
    let permutations = dir_to_chars(dir);
    let new_moves = permutations.into_iter().filter_map(|p| {
      if pos == 'A' && p.len() >= 2 && p[0] == '<' && p[1] == '<' {
        return None;
      }
      if pos == '0' && p.len() >= 1 && p[0] == '<' {
        return None;
      }
      if pos == '1' && p.len() >= 1 && p[0] == 'v' {
        return None;
      }
      if pos == '4' && p.len() >= 2 && p[0] == 'v' && p[1] == 'v' {
        return None;
      }
      if pos == '7' && p.len() >= 3 && p[0] == 'v' && p[1] == 'v' && p[2] == 'v' {
        return None;
      }
      let mut m = moves.clone();
      let mut p = p.clone();
      p.push('A');
      m.iter_mut().for_each(|m| m.extend(p.clone()));
      Some(m)
    }).flatten().collect::<Vec<Vec<char>>>();
    moves = new_moves;
    pos = c;
  }

  moves.into_iter().collect::<HashSet<Vec<char>>>()
}

fn arrowpad(pos: char, c: char, level: u32, cache: &mut HashMap<(char, char, u32), u64>) -> u64 {
  if let Some(x) = cache.get(&(pos, c, level)) {
    return *x;
  }

    let options = match (pos, c) {
          ('A', '^') => vec![vec!['<', 'A']],
          ('A', '<') => vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']],
          ('A', '>') => vec![vec!['v', 'A']],
          ('A', 'v') => vec![vec!['<', 'v', 'A'], vec!['v', '<', 'A']],
          ('<', 'A') => vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']],
          ('<', 'v') => vec![vec!['>', 'A']],
          ('<', '^') => vec![vec!['>', '^', 'A']],
          ('^', 'A') => vec![vec!['>', 'A']],
          ('^', '>') => vec![vec!['v', '>', 'A'], vec!['>', 'v',  'A']],
          ('^', '<') => vec![vec!['v', '<', 'A']],
          ('>', '^') => vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']],
          ('>', 'v') => vec![vec!['<', 'A']],
          ('>', 'A') => vec![vec!['^', 'A']],
          ('v', '<') => vec![vec!['<', 'A']],
          ('v', '>') => vec![vec!['>', 'A']],
          ('v', 'A') => vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']],
          (x, y) => {
            if x == y {
              vec![vec!['A']]
            } else {
              panic!("Unknown move: {} -> {}", x, y) 
            }
          }
        };

  let ret = if level == 0 {
    options[0].len() as u64
  } else {
    options.into_iter().map(|mv| {
      let mv = vec!['A'].into_iter().chain(mv.iter().cloned()).collect::<Vec<char>>();
      mv.windows(2).map(|c| {
        arrowpad(c[0], c[1], level-1, cache)
      }).sum()
    }).min().unwrap()
  };

  cache.insert((pos, c, level), ret);
  ret
}

fn dir_to_chars(dir: (i32, i32)) -> Vec<Vec<char>> {
  let mut chars = Vec::new();

   for _ in 0..dir.0.abs() {
        if dir.0 > 0 {
          chars.push('>');
        } else {
          chars.push('<');
        }
    }
      for _ in 0..dir.1.abs() {
        if dir.1 > 0 {
          chars.push('v');
        } else {
          chars.push('^');
        }
    }
  let l = chars.len();
  chars.into_iter().permutations(l).collect::<Vec<Vec<char>>>()
}


use std::fs;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;
use hashbrown::HashMap;
use itertools::Itertools;
  
fn main() {
    let input = fs::read_to_string("data/day21.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
  input.lines().map(|l| {
    let len = solve(l);
    let numeric = l.strip_suffix("A").unwrap().parse::<u32>().unwrap();
    len * numeric
  }).sum()
}

fn solve(input: &str) -> u32 {
  let numpad = Box::new(Numpad{});
  let arrowpad = Box::new(Arrowpad{pad: numpad});
  let arrowpad = Box::new(Arrowpad{pad: arrowpad});

  let chars = input.chars().collect::<Vec<char>>();

  let res = arrowpad.push(&chars);
  res.iter().map(|m| m.len()).min().unwrap() as u32
}

trait Pad {
  fn push(&self, chars: &Vec<char>) -> Vec<Vec<char>>;
  fn position(&self, c: char) -> (i32, i32);
}

struct Numpad {
}

impl Pad for Numpad {
  fn position(&self, c: char) -> (i32, i32) {
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

  fn push(&self, chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut pos = 'A';
    let mut moves = vec![Vec::new()];

    for c in chars {
      let dir = (self.position(*c).0 - self.position(pos).0, self.position(*c).1 - self.position(pos).1);
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
      pos = *c;
    }

    moves
  }
}

struct Arrowpad {   
  pub pad: Box<dyn Pad>,
}


impl Pad for Arrowpad {
  fn position(&self, c: char) -> (i32, i32) {
    match c {
      '^' => (1, 0),
      'v' => (1, 1),
      '<' => (0, 1),
      '>' => (2, 1),
      'A' => (2, 0),
      _ => panic!("Invalid position: {}", c),
    }
  }

  fn push(&self, chars: &Vec<char>) -> Vec<Vec<char>> {
    let possible_chars = self.pad.push(chars); //FIXME this is now a vec of vecs
    let mut possible_moves = Vec::new();

    for chars in possible_chars {
      let mut pos = 'A';
      let mut moves = vec![];

      for c in chars {
        let m = match (pos, c) {
          ('A', '^') => vec!['<', 'A'],
          ('A', '<') => vec!['v', '<', '<', 'A'],
          ('A', '>') => vec!['v', 'A'],
          ('A', 'v') => vec!['<', 'v', 'A'],
          ('<', 'A') => vec!['>', '>', '^', 'A'],
          ('<', 'v') => vec!['>', 'A'],
          ('<', '^') => vec!['>', '^', 'A'],
          ('^', 'A') => vec!['>', 'A'],
          ('^', '>') => vec!['>', 'v', 'A'],
          ('^', '<') => vec!['v', '<', 'A'],
          ('>', '^') => vec!['^', '<', 'A'],
          ('>', 'v') => vec!['<', 'A'],
          ('>', 'A') => vec!['^', 'A'],
          ('v', '<') => vec!['<', 'A'],
          ('v', '>') => vec!['>', 'A'],
          ('v', 'A') => vec!['>', '^', 'A'],
          (x, y) => {
            if x == y {
              vec!['A']
            } else {
              panic!("Unknown move: {} -> {}", x, y) 
            }
          }
        };
        moves.extend(m);
        pos = c;
      }
      possible_moves.push(moves);
    };

    let shortest_len = possible_moves.iter().map(|m| m.len()).min().unwrap();
    possible_moves.retain(|m| m.len() == shortest_len);
    possible_moves
  }
}

static DIR_TO_CHAR_CACHE: LazyLock<Mutex<HashMap<(i32, i32), Vec<Vec<char>>>>> = LazyLock::new(|| {
  Mutex::new(HashMap::new())
});
fn dir_to_chars(dir: (i32, i32)) -> Vec<Vec<char>> {
  if let Some(x) = DIR_TO_CHAR_CACHE.lock().unwrap().get(&dir) {
    return x.clone();
  }

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
  let r = chars.into_iter().permutations(l).collect::<Vec<Vec<char>>>();
  DIR_TO_CHAR_CACHE.lock().unwrap().insert(dir, r.clone());
  r
}


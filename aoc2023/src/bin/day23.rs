use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day23.txt")
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
    let (grid, dims) = parse(input);

    let start = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == 0)
        .next().unwrap().0;

    let target = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == dims.0)
        .next().unwrap().0;

    let mut visited = HashSet::new();
    longest_possible_path(&grid, start, target, visited)

}


fn part2(input: &str) -> usize {
    let (grid, dims) = parse(input);

    let start = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == 0)
        .next().unwrap().0;

    let target = *grid.iter()
        .filter(|(pos, &c)| c == '.' && pos.0 == dims.0)
        .next().unwrap().0;

    let grid = grid.iter()
        .map(|(pos, &c)| (*pos, if c == '#' {c} else {'.'}))
        .collect::<HashMap<_, _>>();

    let mut intersections = intersection(&grid);
    intersections.push(start);
    intersections.push(target);

    let collapsed = collapse(&grid, &intersections);

    longest_possible_path_collapsed(&collapsed, start, target, vec![])
}

fn parse(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize)) {
    let grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    ((i as isize, j as isize), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(isize, isize), char>>();

    let dims = (*grid.keys().map(|(i, _)| i).max().unwrap(), *grid.keys().map(|(_, j)| j).max().unwrap());

    (grid, dims)
}

fn longest_possible_path(grid: &HashMap<(isize, isize), char>, pos: (isize, isize), target: (isize, isize), visited: HashSet<(isize, isize)>) -> usize {
    let mut visited = visited.clone();
    visited.insert(pos);

    let dirs = match grid[&pos] {
        '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        _ => unreachable!()
    };

    dirs.iter()
        .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
        .filter(|&next_pos| grid.contains_key(&next_pos) && grid[&next_pos] != '#' && !visited.contains(&next_pos))
        .map(|next_pos| {
            if next_pos == target {
                visited.len()
            } else {
                longest_possible_path( grid, next_pos, target, visited.clone())
            }
        })
        .max()
        .unwrap_or(0)
}

fn collapse(grid: &HashMap<(isize, isize), char>, intersections: &Vec<(isize, isize)>) -> HashMap<(isize, isize), HashMap<(isize, isize), usize>> {
    let mut collapsed = intersections.iter()
        .map(|pos| (*pos, HashMap::<(isize, isize), usize>::new()))
        .collect::<HashMap<_, _>>();

    //for each candidate position, do a BFS to find the distance to all other reachable candidates
    for &pos in intersections {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((pos, 0));

        while !queue.is_empty() {
            let (new_pos, dist) = queue.pop_front().unwrap();
            visited.insert(new_pos);

            if collapsed.contains_key(&new_pos) && new_pos != pos {
                collapsed.get_mut(&pos).unwrap().insert(new_pos, dist);
                continue;
            }

            for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
                if grid.contains_key(&next_pos) && grid[&next_pos] != '#' && !visited.contains(&next_pos) {
                    queue.push_back((next_pos, dist + 1));
                }
            }
        }
    }

    collapsed
}

fn intersection(grid: &HashMap<(isize, isize), char>) -> Vec<(isize, isize)> {
    grid.iter()
        .filter(|(pos, c)| **c != '#')
        .filter(|(pos, _)| {
            let neighbours_count = vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
                .filter(|&next_pos| grid.contains_key(&next_pos) && grid[&next_pos] != '#')
                .count();
            neighbours_count >= 3
        })
        .map(|(pos, _)| *pos)
        .collect()
}


fn longest_possible_path_collapsed(grid: &HashMap<(isize, isize), HashMap<(isize, isize), usize>>, pos: (isize, isize), target: (isize, isize), visited: Vec<(isize, isize)>) -> usize {
    let mut visited = visited.clone();
    visited.push(pos);

    grid[&pos].iter()
        .filter(|(next_pos, dist)| !visited.contains(&next_pos))
        .map(|(next_pos, dist)| {
            if *next_pos == target {
                let mut visited = visited.clone();
                visited.push(target);
                visited.iter()
                    .tuple_windows()
                    .map(|(pos1, pos2)| grid[pos1][pos2])
                    .sum::<usize>()
            } else {
                longest_possible_path_collapsed(grid, *next_pos, target, visited.clone())
            }
        })
        .max()
        .unwrap_or(0)
}
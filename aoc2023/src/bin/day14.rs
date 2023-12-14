use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day14.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    let mut grid = input.split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim_i = grid.len();
    let dim_j = grid.get(0).unwrap().len();

    for i in 0..dim_i {
        for j in 0..dim_j  {
            if get(&grid, (i, j)) != 'O' {
                continue;
            }

            let new_pos = tilt(&grid, (i, j));
            *grid.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
            *grid.get_mut(new_pos.0).unwrap().get_mut(new_pos.1).unwrap() = 'O';
        }
    }

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|c| **c == 'O').count() * (dim_i-i)
        })
        .sum()
}

fn get(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> char {
    grid.get(pos.0).unwrap().get(pos.1).unwrap().clone()
}

fn tilt(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> (usize, usize) {
    if pos.0 == 0 {
        return pos;
    }

    let mut pos = pos;

    loop {
        let next = (pos.0 - 1, pos.1);
        if pos.0 == 0 {
            return pos;
        }
        if get(grid, next) == '.' {
            pos = next;
        } else {
            break;
        }
    }

    return pos;
}
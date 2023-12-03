use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("data/day03.txt")
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
    let mut grid = HashMap::<(usize, usize), char>::new();

    input.split("\n").enumerate()
        .for_each(|(i, line)| {
           line.chars()
               .enumerate()
               .for_each(|(j, c)| {
                   grid.insert((i+1, j+1), c);
               })
        });

    grid.iter()
        .filter(|(&(i, j), &c)| {
            c.is_digit(10) && !grid.get(&(i, j - 1)).unwrap_or(&'.').is_digit(10)
        })
        .map(|(&(i, j), &c)| {
            let mut digits = Vec::<char>::new();
            digits.push(c);
            for idx in 1..5 {
                let d = grid.get(&(i, j+idx)).unwrap_or(&'.');
                if !d.is_digit(10) {
                    break;
                }
                digits.push(*d);
            }
            ((i, j), digits.iter().collect::<String>())
        })
        .filter(|((i, j), d)| {
            grid.iter()
                .filter(|(&(ii, jj), _)| {
                    ii >= i-1 && ii <= i+1 && jj >= j-1 && jj <=j+d.len()
                })
                .any(|(_, c)| {
                    !(c.is_digit(10) || *c == '.')
                })
        })
        .map(|(_, s)| {
            s.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

use std::fs;
use itertools::Itertools;
use std::ops::Add;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input.split("\n")
        .map(designator_to_seat_id)
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let seat_ids :Vec<i32> = input.split("\n")
        .map(designator_to_seat_id)
        .collect::<Vec<i32>>()
        .into_iter()
        .sorted()
        .collect();

    (1..seat_ids.len() - 1)
        .filter(|i| seat_ids[*i] == seat_ids[i + 1] - 2)
        .map(|i| seat_ids[i])
        .collect::<Vec<i32>>()
        .first()
        .unwrap()
        .add(1)
}

fn designator_to_seat_id(designator :&str) -> i32 {
    let designator :String = designator.chars()
        .map(|ch| match ch {
            'R' => '1',
            'L' => '0',
            'B' => '1',
            'F' => '0',
            _ => unreachable!()
        })
        .collect();
    return isize::from_str_radix(&*designator, 2).unwrap() as i32;
}
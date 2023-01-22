use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut tree_count = 0;
    let slope = 3;
    let mut pos_x = 0;
    for line in input.split("\n") {
        let ch = line.chars().collect::<Vec<char>>()[pos_x];
        if ch == '#' {
            tree_count += 1;
        }
        pos_x = (pos_x + slope) % line.len();
    }
    tree_count
}

fn part2(input: &str) -> i64 {
    let mut tree_counts = [0, 0, 0, 0, 0];
    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    for (slope, tree_count) in slopes.iter().zip(tree_counts.iter_mut()) {
        let mut pos_x = 0;
        for line in input.split("\n").step_by(slope[1]) {
            let ch = line.chars().collect::<Vec<char>>()[pos_x];
            if ch == '#' {
                *tree_count += 1;
            }
            pos_x = (pos_x + slope[0]) % line.len();
        }
    }
    tree_counts.iter().product()
}
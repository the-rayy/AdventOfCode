use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day14.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn part1(input: &str) -> usize {
    let iters = 100;
    let robots = input.lines().map(|line| {
        let line = &line.replace(" v=", ",")[2..];
        let mut line = line.split(",");
        let pos = (
            line.next().unwrap().parse::<i64>().unwrap(),
            line.next().unwrap().parse::<i64>().unwrap(),
        );
        let vel = (
            line.next().unwrap().parse::<i64>().unwrap(),
            line.next().unwrap().parse::<i64>().unwrap(),
        );

        simulate(pos, vel, iters)
    });

    let q1 = robots
        .clone()
        .filter(|(x, y)| *x < WIDTH / 2 && *y < HEIGHT / 2)
        .count();
    let q2 = robots
        .clone()
        .filter(|(x, y)| *x > WIDTH / 2 && *y < HEIGHT / 2)
        .count();
    let q3 = robots
        .clone()
        .filter(|(x, y)| *x < WIDTH / 2 && *y > HEIGHT / 2)
        .count();
    let q4 = robots
        .clone()
        .filter(|(x, y)| *x > WIDTH / 2 && *y > HEIGHT / 2)
        .count();

    q1 * q2 * q3 * q4
}

fn part2(input: &str) -> usize {
    let iters = 8270; // Found by pen and paper
                      // By printing the grid you can see some partially ordered images here and there.
                      // For my input there was two cycles:
                      // 1. Horizontal lines on iters: 30, 133, 239..., -> diff=103
                      // 2. Vertical lines on iters: 89, 190, 291..., -> diff=101
                      //
                      // So by using chinese remainder theorem we can find the time when the image is ordered.
                      // t = 8270 (mod 10403)

    let robots = input
        .lines()
        .map(|line| {
            let line = &line.replace(" v=", ",")[2..];
            let mut line = line.split(",");
            let pos = (
                line.next().unwrap().parse::<i64>().unwrap(),
                line.next().unwrap().parse::<i64>().unwrap(),
            );
            let vel = (
                line.next().unwrap().parse::<i64>().unwrap(),
                line.next().unwrap().parse::<i64>().unwrap(),
            );

            simulate(pos, vel, iters)
        })
        .collect::<Vec<_>>();

    draw(&robots);
    iters as usize
}

fn simulate(pos: (i64, i64), vel: (i64, i64), time: i64) -> (i64, i64) {
    let x = (pos.0 + vel.0 * time).rem_euclid(WIDTH);
    let y = (pos.1 + vel.1 * time).rem_euclid(HEIGHT);

    (x, y)
}

fn draw(robots: &[(i64, i64)]) {
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    for pos in robots.iter() {
        grid[pos.1 as usize][pos.0 as usize] = '#';
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

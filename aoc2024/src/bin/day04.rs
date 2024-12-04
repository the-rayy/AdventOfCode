use std::fs;
use std::time::Instant;
use hashbrown::HashMap;


fn main() {
    let input = fs::read_to_string("data/day04.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

//    let part2_start = Instant::now();
//    let part2_ans = part2(&input);
//   println!("Part 2 time: {:.2?}", part2_start.elapsed());
//    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let crossword: HashMap<(i32, i32), char> = input.lines().enumerate()
        .map(|(i, line)| {
            line.chars().enumerate()
                .map(move |(j, c)| {
                    ((i as i32, j as i32), c)
                })
        }).flatten().collect();
    let dirs = vec![
        (0, 1), (1, 0), (0, -1), (-1, 0),
        (1, 1), (1, -1), (-1, 1), (-1, -1)
    ];

    let mut score = 0;
    crossword.iter().filter(|(_, &c)| c == 'X')
        .for_each(|(pos, _)| {
            for dir in &dirs {
                if crossword.get(&(pos.0 + dir.0*1, pos.1 + dir.1*1)).unwrap_or(&' ') == &'M' &&
                   crossword.get(&(pos.0 + dir.0*2, pos.1 + dir.1*2)).unwrap_or(&' ') == &'A' &&
                   crossword.get(&(pos.0 + dir.0*3, pos.1 + dir.1*3)).unwrap_or(&' ') == &'S' {
                    score += 1;
                };
            }
        });

    score
}


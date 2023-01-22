use std::fs;

const BOUND :char = ' ';
const FLOOR :char = '.';
const EMPTY :char = 'L';
const TAKEN :char = '#';
static NEIGH_DIFF :&'static [i32] = &[-1, 1, -100, -99, -98, 98, 99, 100];
static DIRECTIONS :&'static [(i32, i32)] = &[(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut tmp :Vec<Vec<char>> = Vec::new();
    tmp.push([BOUND; 100].to_vec());
    for line in input.split("\n") {
        let mut tmp2 :Vec<char> = Vec::new();
        tmp2.push(BOUND);
        tmp2.append(&mut line.chars().collect::<Vec<char>>());
        tmp2.push(BOUND);
        tmp.push(tmp2);
    }
    tmp.push([BOUND; 100].to_vec());
    let mut seats = tmp.into_iter().flatten().collect();

    loop {
        let new_seats = step(&seats);
        let taken = new_seats.iter().filter(|x| **x == TAKEN).count() as i64;
        if seats.iter()
            .zip(&new_seats)
            .all(|(a,b)| *a == *b) {
            return taken;
        }
        seats = new_seats
    }
}

fn part2(input: &str) -> i64 {
    let mut seats:Vec<Vec<char>> = Vec::new();
    seats.push([BOUND; 100].to_vec());
    for line in input.split("\n") {
        let mut tmp:Vec<char> = Vec::new();
        tmp.push(BOUND);
        tmp.append(&mut line.chars().collect::<Vec<char>>());
        tmp.push(BOUND);
        seats.push(tmp);
    }
    seats.push([BOUND; 100].to_vec());

    loop {
        let new_seats = step2(&seats);
        let taken = new_seats.iter().flatten().filter(|x| **x == TAKEN).count() as i64;
        if seats.iter()
            .zip(&new_seats)
            .all(|(a,b)| *a == *b) {
            return taken;
        }
        seats = new_seats
    }
}

fn step(input :&Vec<char>) -> Vec<char> {
    let mut output = input.iter().copied().collect::<Vec<char>>();

    for i in 0 .. input.len() {
        if input[i] == BOUND {
            continue
        }
        let neighbours = NEIGH_DIFF.iter()
            .map(|diff| (i as i32) + diff)
            .map(|idx| input.get(idx as usize))
            .map(|x| *x.unwrap())
            .collect::<Vec<char>>();
        if input[i] == EMPTY && neighbours.iter().filter(|x| **x == TAKEN).count() == 0 {
            output[i] = TAKEN;
        }
        if input[i] == TAKEN && neighbours.iter().filter(|x| **x == TAKEN).count() >= 4 {
            output[i] = EMPTY;
        }
    }
    output
}

fn step2(input :&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for i in 00 .. input.len() {
        output.push(input[i].iter().copied().collect::<Vec<char>>());
    }

    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if input[i][j] == BOUND {
                continue
            }
            let neighbours = DIRECTIONS.iter()
                .map(|dir| first_in_direction(input, (i as i32, j as i32), *dir))
                .collect::<Vec<char>>();
            if input[i][j] == EMPTY && neighbours.iter().filter(|x| **x == TAKEN).count() == 0 {
                output[i][j] = TAKEN;
            }
            if input[i][j] == TAKEN && neighbours.iter().filter(|x| **x == TAKEN).count() >= 5 {
                output[i][j] = EMPTY;
            }
        }
    }
    output
}

fn first_in_direction(input :&Vec<Vec<char>>, target :(i32, i32), dir :(i32, i32)) -> char {
    let mut distance = 1;
    loop {
        let x :usize = (target.0 + dir.0 * distance) as usize;
        let y :usize = (target.1 + dir.1 * distance) as usize;
        match input[x][y] {
            FLOOR => distance +=1,
            _ => return input[x][y]
        }
    }
}
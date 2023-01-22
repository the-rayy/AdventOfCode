use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let instructions = input.split("\n")
        .map(|instr| instr.split(" ")
            .collect::<Vec<&str>>())
        .map(|instr| (instr[0], instr[1].parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();

    let (acc, _) = halts(&instructions);
    acc
}

fn part2(input: &str) -> i32 {
    let instructions = input.split("\n")
        .map(|instr| instr.split(" ")
            .collect::<Vec<&str>>())
        .map(|instr| (instr[0], instr[1].parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();

    for i in 0 .. instructions.len() {
        if instructions[i].0 == "acc" {
            continue
        }

        let mut cp = instructions.iter().copied().collect::<Vec<(&str, i32)>>();
        match cp[i].0 {
            "nop" => cp[i] = ("jmp", cp[i].1),
            "jmp" => cp[i] = ("nop", cp[i].1),
            _ => unreachable!()
        }

        let (acc, stopped) = halts(&cp);
        if stopped {
            return acc
        }
    }
    unreachable!()
}

fn halts(instructions: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut ptr: i32 = 0;
    let mut acc: i32 = 0;
    let mut ptrs: HashSet<i32> = HashSet::new();
    loop {
        if ptrs.contains(&ptr) {
            return (acc, false)
        }
        ptrs.insert(ptr);

        if step(&instructions, &mut ptr, &mut acc) {
            return (acc, true)
        }
    };
}

fn step(instructions: &Vec<(&str, i32)>, ptr: &mut i32, acc: &mut i32) -> bool {
    if *ptr as usize >= instructions.len() {
        return true
    }
    match instructions[*ptr as usize].0 {
        "acc" => {
            *acc += instructions[*ptr as usize].1;
            *ptr += 1;
        }
        "jmp" => {
            *ptr += instructions[*ptr as usize].1;
        }
        "nop" => {
            *ptr += 1;
        }
        _ => unreachable!()
    }
    false
}

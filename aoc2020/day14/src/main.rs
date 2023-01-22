use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i128 {
    let mut mask_repl :u64 = 0;
    let mut mask_x :u64 = 0;
    let mut memory :HashMap<u64, u64> = HashMap::new();
    input.split("\n")
        .for_each(|line| {
            match &line[0..4] {
                "mask" => {
                    let tmp = read_mask(line);
                    mask_repl = tmp.0;
                    mask_x = tmp.1
                },
                "mem[" => read_mem(line, mask_repl, mask_x, &mut memory),
                _ => {}
            }
        });
    memory.values().map(|x| *x as i128).sum()
}

fn read_mask(line: &str) -> (u64, u64) {
    let mask_x = line[7..line.len()].chars()
        .map(|ch| match ch {
            '1' => '0',
            '0' => '0',
            'X' => '1',
            _ => unreachable!()
        })
        .collect::<String>();
    let mask_x = isize::from_str_radix(&*mask_x, 2).unwrap() as u64;

    let mask_repl = line[7..line.len()].chars()
        .map(|ch| match ch {
            '1' => '1',
            '0' => '0',
            'X' => '0',
            _ => unreachable!()
        })
        .collect::<String>();
    let mask_repl = isize::from_str_radix(&*mask_repl, 2).unwrap() as u64;

    (mask_repl, mask_x)
}

fn read_mem(line :&str, mask_repl :u64, mask_x :u64, memory: &mut HashMap<u64, u64>) {
    let re: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    let caps = re.captures(line).unwrap();
    let addr :u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let val :u64 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
    let masked = (val & mask_x) + mask_repl;

    memory.insert(addr, masked);
}

fn part2(input: &str) -> i128 {
    let mut masks_repl :Vec<u64> = Vec::new();
    let mut masks_x :Vec<u64> = Vec::new();
    let mut memory :HashMap<u64, u64> = HashMap::new();
    input.split("\n")
        .for_each(|line| {
            match &line[0..4] {
                "mask" => {
                    let tmp = read_mask2(line);
                    masks_repl = tmp.0;
                    masks_x = tmp.1
                },
                "mem[" => read_mem2(line, &masks_repl, &masks_x, &mut memory),
                _ => {}
            }
        });
    memory.values().map(|x| *x as i128).sum()
}

fn expand(mask :String) -> Vec<String> {
    if !mask.contains("F") {
        return vec![mask]
    };

    let mut ret :Vec<String> = Vec::new();
    let repl_0 = mask.replacen("F", "0", 1);
    let repl_1 = mask.replacen("F", "1", 1);
    ret.extend(expand(repl_0));
    ret.extend(expand(repl_1));
    ret
}

fn read_mask2(line: &str) -> (Vec<u64>, Vec<u64>) {
    let translated = line[7..line.len()].chars()
        .map(|ch| match ch {
            '1' => '1',
            '0' => 'X',
            'X' => 'F',
            _ => unreachable!()
        })
        .collect::<String>();
    let translateds = expand(translated);

    let mut masks_repl :Vec<u64> = Vec::new();
    let mut masks_x :Vec<u64> = Vec::new();

    translateds.iter()
        .for_each(|t| {
            let mask_x = t.chars()
                .map(|ch| match ch {
                    '1' => '0',
                    '0' => '0',
                    'X' => '1',
                    _ => unreachable!()
                })
                .collect::<String>();
            masks_x.push(isize::from_str_radix(&*mask_x, 2).unwrap() as u64);

            let mask_repl = t.chars()
                .map(|ch| match ch {
                    '1' => '1',
                    '0' => '0',
                    'X' => '0',
                    _ => unreachable!()
                })
                .collect::<String>();
            masks_repl.push(isize::from_str_radix(&*mask_repl, 2).unwrap() as u64);
        });

    (masks_repl, masks_x)
}

fn read_mem2(line :&str, masks_repl :&Vec<u64>, masks_x :&Vec<u64>, memory: &mut HashMap<u64, u64>) {
    let re: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    let caps = re.captures(line).unwrap();
    let addr :u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let val :u64 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
    masks_repl.iter()
        .zip(masks_x)
        .for_each(|(mask_repl, mask_x)| {
            let masked = (addr & *mask_x) + *mask_repl;
            memory.insert(masked, val);
    });
}


use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut correct = 0;
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for passport_raw in input.split("\n\n") {
        let passport_raw = passport_raw.replace("\n", " ");
        let passport_entries = passport_raw.split(" ");
        let mut passport :HashMap<&str, &str> = HashMap::new();
        passport_entries.for_each(|entry| {
            let en :Vec<&str> = entry.split(":").collect();
            passport.insert(en[0], en[1]);
        });
        let passport_keys :Vec<&&str> = passport.keys().collect();
        if required_keys.iter().all(|item| passport_keys.contains(&item)) {
            correct += 1;
        };
    }
    correct
}

fn part2(input: &str) -> i32 {
    let mut correct = 0;
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for passport_raw in input.split("\n\n") {
        let passport_raw = passport_raw.replace("\n", " ");
        let passport_entries = passport_raw.split(" ");
        let mut passport :HashMap<&str, &str> = HashMap::new();
        passport_entries.for_each(|entry| {
            let en :Vec<&str> = entry.split(":").collect();
            passport.insert(en[0], en[1]);
        });

        let passport_keys :Vec<&&str> = passport.keys().collect();
        if !required_keys.iter().all(|item| passport_keys.contains(&item)) {
            continue
        }

        let byr:i32 = passport["byr"].parse().unwrap();
        if byr < 1920 || byr > 2002 {
            continue
        }

        let iyr:i32 = passport["iyr"].parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            continue
        }

        let eyr:i32 = passport["eyr"].parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            continue
        }

        if passport["hgt"].ends_with("cm") {
            let chars = passport["hgt"].chars().collect::<Vec<char>>();
            let hgt_val :i32 = chars[0 .. chars.len()-2].iter().collect::<String>().parse().unwrap();
            if hgt_val < 150 || hgt_val > 193 {
                continue
            }
        }
        if passport["hgt"].ends_with("in") {
            let chars = passport["hgt"].chars().collect::<Vec<char>>();
            let hgt_val :i32 = chars[0 .. chars.len()-2].iter().collect::<String>().parse().unwrap();
            if hgt_val < 59 || hgt_val > 76 {
                continue
            }
        }
        if !passport["hgt"].ends_with("in") && ! passport["hgt"].ends_with("cm") {
            continue
        }

        let re: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        match re.find(passport["hcl"]) {
            Some(_) => (),
            None    => continue
        }

        if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"]) {
            continue
        }

        let re: Regex = Regex::new(r"^\d{9}$").unwrap();
        match re.find(passport["pid"]) {
            Some(_) => (),
            None    => continue
        }

        correct += 1;
    }
    correct
}
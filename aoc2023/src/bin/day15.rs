use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day15.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    input.split(",")
        .map(hash)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

    for line in input.split(",") {
        if !line.contains("=") {
            let mut parts = line.split("-");
            let label = parts.next().unwrap();
            let h = hash(label);
            if let Some(idx) = boxes[h].iter().position(|x| x.label == label) {
                boxes[h].remove(idx);
            }
            continue;
        }

        let mut parts = line.split("=");
        let label = parts.next().unwrap();
        let focal_length = parts.next().unwrap().parse::<usize>().unwrap();
        let h = hash(label);
        if let Some(idx) = boxes[h].iter().position(|x| x.label == label) {
            boxes[h][idx].focal_length = focal_length;
        } else {
            boxes[h].push(Lens { label: label.to_string(), focal_length });
        }
    }

    boxes.iter()
        .enumerate()
        .map(|(box_id, lenses)| {
            lenses.iter()
                .enumerate()
                .map(|(lens_id, lens)| {
                    (1 + box_id) * (1 + lens_id) * lens.focal_length
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .sum()
}

#[derive(Debug, Default)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn hash(input: &str) -> usize {
    input.chars()
        .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}
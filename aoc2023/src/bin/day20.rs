use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;
use num::integer::lcm;

fn main() {
    let input = fs::read_to_string("data/day20.txt")
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


fn part1(input: &str) -> u64 {
    let (mut modules, connections) = parse(input);
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let (h, l) = press_button(&mut modules, &connections);
        high += h;
        low += l;
    }


    low * high
}

fn part2(_: &str) -> u64 {
    // pen and paper solution
    [3881_u64, 3851_u64, 3943_u64, 3931_u64].into_iter()
        .fold(1_u64, |acc, x| lcm(acc, *x) )
}

fn press_button(modules: &mut HashMap<String, Box<dyn Module>>, connections: &HashMap<String, Vec<String>>) -> (u64, u64) {
    let mut high: u64 = 0;
    let mut low: u64 = 0;

    let mut q = VecDeque::<(String, String, Pulse)>::new();
    q.push_back((String::from("button"), String::from("broadcaster"), Pulse::Low));

    while let Some((source, destination, pulse)) = q.pop_front() {
        match pulse {
            Pulse::High => high += 1,
            Pulse::Low => low += 1,
        }
        let mut module = modules.get_mut(destination.as_str());
        if module.is_none() {
            continue;
        }
        let mut module = module.unwrap();
        let new_pulse = module.pulse(source.as_str(), pulse);

        if new_pulse.is_none() {
            continue;
        }

        for output in connections.get(destination.as_str()).unwrap() {
            q.push_back((destination.clone(), output.clone(), new_pulse.unwrap()));
        }
    }

    (high, low)
}

fn parse(input: &str) -> (HashMap<String, Box<dyn Module>>, HashMap<String, Vec<String>>) {
    let mut modules = HashMap::<String, Box<dyn Module>>::new();
    let mut connections = HashMap::<String, Vec<String>>::new();

    for line in input.split("\n") {
        let mut splitted = line.split(" -> ");
        let source = splitted.next().unwrap();
        let dest = splitted.next().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        if source.starts_with("%") {
            let source = source.strip_prefix("%").unwrap();
            modules.insert(source.to_string(), Box::new(FlipFlop::new()));
            connections.insert(source.to_string(), dest);
        } else if source.starts_with("&") {
            let source = source.strip_prefix("&").unwrap();
            modules.insert(source.to_string(), Box::new(Conjunction::new()));
            connections.insert(source.to_string(), dest);
        } else {
            modules.insert(source.to_string(), Box::new(Broadcast::new()));
            connections.insert(source.to_string(), dest);
        }
    }

    for (input, outputs) in connections.iter() {
        for o in outputs {
            if let Some(x) = modules.get_mut(o) {
                x.update_inputs(input.to_string());
            }
        }
    }

    (modules, connections)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Pulse {
    Low,
    High,
}

trait Module {
    fn pulse(&mut self, source: &str, pulse: Pulse) -> Option<Pulse>;
    fn update_inputs(&mut self, input: String);
    fn all_inputs(&self) -> Option<bool> {
        None
    }
}

#[derive(Clone)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn new() -> FlipFlop {
        FlipFlop {
            state: false,
        }
    }
}

impl Module for FlipFlop {
    fn pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        if pulse == Pulse::High {
            return None;
        }

        if self.state {
            self.state = false;
            Some(Pulse::Low)
        } else {
            self.state = true;
            Some(Pulse::High)
        }
    }

    fn update_inputs(&mut self, _: String) {

    }
}

#[derive(Clone)]
struct Conjunction {
    mem: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Conjunction {
        Conjunction {
            mem: HashMap::new(),
        }
    }
}

impl Module for Conjunction {
    fn pulse(&mut self, source: &str, pulse: Pulse) -> Option<Pulse> {
        self.mem.insert(source.to_string(), pulse.clone());
        if self.mem.values().all(|&p| p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }


    fn update_inputs(&mut self, input: String) {
        self.mem.insert(input, Pulse::Low);
    }

    fn all_inputs(&self) -> Option<bool> {
        Some(self.mem.values().all(|&p| p == Pulse::High))
    }
}

#[derive(Clone)]
struct Broadcast {
}

impl Broadcast {
    fn new() -> Broadcast {
        Broadcast {
        }
    }
}

impl Module for Broadcast {
    fn pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse.clone())
    }

    fn update_inputs(&mut self, _: String) {

    }
}


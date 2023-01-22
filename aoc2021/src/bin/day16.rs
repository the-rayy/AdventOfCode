use std::fs;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    length_type_id: u64,
    value: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn sumver(&self) -> u64 {
        self.subpackets.iter()
            .map(|sub| sub.sumver())
            .sum::<u64>() + self.version
    }

    fn calc(&self) -> u64 {
        match self.type_id {
            0 => {
                self.subpackets.iter().map(|p| p.calc()).sum::<u64>()
            }
            1 => {
                self.subpackets.iter().fold(1u64, |prod, p| prod * p.calc() )
            }
            2 => {
                self.subpackets.iter().map(|p| p.calc()).min().unwrap()
            }
            3 => {
                self.subpackets.iter().map(|p| p.calc()).max().unwrap()
            }
            4 => {
                self.value
            }
            5 => {
                if self.subpackets[0].calc() > self.subpackets[1].calc() { 1 } else { 0 }
            }
            6 => {
                if self.subpackets[0].calc() < self.subpackets[1].calc() { 1 } else { 0 }
            }
            7 => {
                if self.subpackets[0].calc() == self.subpackets[1].calc() { 1 } else { 0 }
            }
            _ => {unreachable!()}
        }
    }
}

fn part1(input: &str) -> i64 {
    let (packet, _) = parse_line(hex_to_bin(input).as_str());
    packet.sumver() as i64
}

fn part2(input: &str) -> i64 {
    let (packet, _) = parse_line(hex_to_bin(input).as_str());
    packet.calc() as i64
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_line(line: &str) -> (Packet, usize) {
    let type_id = str_to_int(&line[3..6]);
    let type_length_id = str_to_int(&line[6..7]);
    let (packet, end) = match (type_id, type_length_id) {
        (4, _) => { parse_literal_packet(line) },
        (_, 0) => { parse_operator0(line) },
        (_, 1) => { parse_operator1(line) },
        (_, _) => unreachable!()
    };
    (packet, end)
}

fn parse_literal_packet(line: &str) -> (Packet, usize) {
    let (value, end) = parse_value(&line[6..]);
    let packet = Packet {
        version: str_to_int(&line[..3]),
        type_id: str_to_int(&line[3..6]),
        length_type_id: 0,
        value: value,
        subpackets: Vec::new(),
    };
    (packet, end+6)
}

fn parse_operator0(line: &str) -> (Packet, usize) {
    let subpacket_len = str_to_int(&line[7..22]) as usize;
    let mut subpackets: Vec<Packet> = Vec::new();
    let mut end: usize = 22;
    while end < 22+subpacket_len{
        let (packet, new_end) = parse_line(&line[end..]);
        end += new_end;
        subpackets.push(packet);
    }
    let packet = Packet {
        version: str_to_int(&line[..3]),
        type_id: str_to_int(&line[3..6]),
        length_type_id: str_to_int(&line[6..7]),
        value: 0,
        subpackets: subpackets,
    };
    (packet, 22+subpacket_len)
}

fn parse_operator1(line: &str) -> (Packet, usize) {
    let subpacket_count = str_to_int(&line[7..18]) as usize;
    let mut subpackets: Vec<Packet> = Vec::new();
    let mut end: usize = 18;
    for _ in 0..subpacket_count {
        let (packet, new_end) = parse_line(&line[end..]);
        subpackets.push(packet);
        end += new_end;
    }

    let packet = Packet {
        version: str_to_int(&line[..3]),
        type_id: str_to_int(&line[3..6]),
        length_type_id: str_to_int(&line[6..7]),
        value: 0,
        subpackets: subpackets,
    };
    (packet, end)
}

fn parse_value(s: &str) -> (u64, usize) {
    let mut binval: String = String::new();
    let mut maxi: usize = 0;
    for i in (0..s.len()).step_by(5) {
        binval += &s[(i + 1)..(i + 5)];
        maxi = i;
        if s.chars().nth(i).unwrap() == '0' { break }
    };
    (str_to_int(binval.as_str()), maxi + 5)
}

fn str_to_int(s: &str) -> u64 {
    isize::from_str_radix(s, 2).unwrap() as u64
}

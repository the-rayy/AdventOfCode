use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input.split("\n")
        .map(|line| line
            .replace("(", "( ")
            .replace(")", " )")
            .split(" ")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
        )
        .map(|x| shunting_yard(&x))
        .map(|x| solve_rpn(&x))
        .sum()
}

fn part2(input: &str) -> i64 {
    input.split("\n")
        .map(|line| line
            .replace("(", "( ")
            .replace(")", " )")
            .split(" ")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
        )
        .map(|x| shunting_yard2(&x))
        .map(|x| solve_rpn(&x))
        .sum()
}

fn shunting_yard(tokens :&Vec<String>) -> Vec<String> {
    let mut output :Vec<&str> = Vec::new();
    let mut opstack :Vec<&str> = Vec::new();

    for token in tokens {
        match token.parse::<i32>() {
            Ok(_) => {
                output.push(token);
            },
            Err(_) => {
                if token == ")" {
                    loop {
                        let last = opstack.pop().unwrap();
                        if last == "(" {
                            break;
                        } else {
                            output.push(last)
                        }
                    }
                } else {
                    loop {
                        if opstack.len() == 0 || *opstack.last().unwrap() == "(" {
                            opstack.push(token);
                            break;
                        }
                        let last = *opstack.last().unwrap();
                        if opprecedence(token) > opprecedence(last) {
                            opstack.push(token);
                            break;
                        } else {
                            output.push(opstack.pop().unwrap());
                        }
                    }
                }
            }
        }
    }

    loop {
        match opstack.pop() {
            Some(x) => output.push(x),
            None => break
        }
    }

    output.into_iter()
        .filter(|&x| x != "(" && x != ")")
        .map(|x| x.to_owned())
        .collect()
}

fn shunting_yard2(tokens :&Vec<String>) -> Vec<String> {
    let mut output :Vec<&str> = Vec::new();
    let mut opstack :Vec<&str> = Vec::new();

    for token in tokens {
        match token.parse::<i32>() {
            Ok(_) => {
                output.push(token);
            },
            Err(_) => {
                if token == ")" {
                    loop {
                        let last = opstack.pop().unwrap();
                        if last == "(" {
                            break;
                        } else {
                            output.push(last)
                        }
                    }
                } else {
                    loop {
                        if opstack.len() == 0 || *opstack.last().unwrap() == "(" {
                            opstack.push(token);
                            break;
                        }
                        let last = *opstack.last().unwrap();
                        if opprecedence2(token) > opprecedence2(last) {
                            opstack.push(token);
                            break;
                        } else {
                            output.push(opstack.pop().unwrap());
                        }
                    }
                }
            }
        }
    }

    loop {
        match opstack.pop() {
            Some(x) => output.push(x),
            None => break
        }
    }

    output.into_iter()
        .filter(|&x| x != "(" && x != ")")
        .map(|x| x.to_owned())
        .collect()
}


fn opprecedence(token :&str) -> usize {
    match token {
        "+" => 10,
        "*" => 10,
        "(" => 999,
        _ => 0
    }
}

fn opprecedence2(token :&str) -> usize {
    match token {
        "+" => 20,
        "*" => 10,
        "(" => 999,
        _ => 0
    }
}

fn solve_rpn(rpn :&Vec<String>) -> i64 {
    let mut stack :Vec<i64> = Vec::new();
    for token in rpn.into_iter() {
        match token.parse::<i64>() {
            Ok(x) => stack.push(x),
            Err(_) => {
                match token.as_str() {
                    "+" => {
                        let val = stack.pop().unwrap() + stack.pop().unwrap();
                        stack.push(val);
                    },
                    "*" => {
                        let val = stack.pop().unwrap() * stack.pop().unwrap();
                        stack.push(val);
                    },
                    _ => unreachable!()
                }
            }
        };
    };

    *stack.first().unwrap()
}
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::BufRead;

enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn try_code(code: &Vec<Op>) -> Option<i32> {
    let mut been: HashSet<usize> = HashSet::new();
    let mut acc = 0;

    let mut pos = 0;

    while !been.contains(&pos) {
        if pos == code.len() {
            return Some(acc);
        } else if pos > code.len() {
            return None;
        }
        been.insert(pos);
        match code[pos] {
            Op::Acc(amount) => {
                acc += amount;
                pos += 1;
            }
            Op::Jmp(amount) => {
                if (pos as i32) < -amount {
                    return None;
                }
                pos = (pos as i32 + amount) as usize;
            }
            _ => {
                pos += 1;
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut code: Vec<Op> = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                let mut line = line.split(" ");

                code.push(match line.next() {
                    Some("acc") => Op::Acc(line.next().unwrap().parse().unwrap()),
                    Some("jmp") => Op::Jmp(line.next().unwrap().parse().unwrap()),
                    Some("nop") => Op::Nop(line.next().unwrap().parse().unwrap()),
                    _ => panic!("Unknown command!"),
                })
            }
        }
    }

    for i in 0..code.len() {
        match code[i] {
            Op::Jmp(amount) => {
                code[i] = Op::Nop(amount);
                if let Some(finish) = try_code(&code) {
                    println!("{}", finish);
                    break;
                }
                code[i] = Op::Jmp(amount);
            }
            Op::Nop(amount) => {
                code[i] = Op::Jmp(amount);
                if let Some(finish) = try_code(&code) {
                    println!("{}", finish);
                    break;
                }
                code[i] = Op::Nop(amount);
            }
            _ => {}
        }
    }

    return Ok(());
}

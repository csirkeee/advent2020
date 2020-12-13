use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::BufRead;

enum Op {
    Acc(i32),
    Jmp(i32),
    Nop,
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
                    _ => Op::Nop,
                })
            }
        }
    }

    let mut been: HashSet<usize> = HashSet::new();
    let mut acc = 0;

    let mut pos = 0;

    while !been.contains(&pos) {
        been.insert(pos);
        match code[pos] {
            Op::Acc(amount) => {
                acc += amount;
                pos += 1;
            }
            Op::Jmp(amount) => {
                pos = (pos as i32 + amount) as usize;
            }
            _ => {
                pos += 1;
            }
        }
    }

    println!("{}", acc);
    return Ok(());
}

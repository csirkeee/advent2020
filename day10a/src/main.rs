use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut v: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse() {
            v.push(i);
        }
    }

    v.sort();

    let mut prev = 0;
    let (mut ones, mut threes) = (0, 1);

    for n in v {
        match n - prev {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
        prev = n;
    }

    println!("{} {} {}", ones, threes, ones * threes);

    return Ok(());
}

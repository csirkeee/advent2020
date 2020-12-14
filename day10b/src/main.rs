use std::cmp::min;
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

    let mut counts: Vec<i64> = Vec::new();

    for i in 0..v.len() {
        let n = v[i];
        let mut count = 0;
        if n <= 3 {
            count += 1
        }
        for back in 0..min(3, i) {
            if v[i - 1 - back] >= n - 3 {
                count += counts[i - 1 - back];
            }
        }
        counts.push(count);
    }

    println!("{}", counts.last().unwrap());

    return Ok(());
}

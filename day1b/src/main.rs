use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut v: Vec<i64> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse() {
            v.push(i);
        }
    }

    for i in 0..v.len() {
        for j in i + 1..v.len() {
            for k in j + 1..v.len() {
                if v[i] + v[j] + v[k] == 2020 {
                    println!("{}", v[i] * v[j] * v[k]);
                    return Ok(());
                }
            }
        }
    }

    return Ok(());
}

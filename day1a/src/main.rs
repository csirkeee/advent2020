use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut v: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse() {
            v.push(i);
        }
    }

    v.sort();

    let (mut i, mut j) = (0, v.len() - 1);

    loop {
        if v[i] + v[j] < 2020 {
            i += 1;
        } else if v[i] + v[j] > 2020 {
            j -= 1;
        } else {
            println!("{}", v[i] * v[j]);
            break;
        }
    }

    return Ok(());
}

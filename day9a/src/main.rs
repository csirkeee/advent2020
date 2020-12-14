use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut v: Vec<i64> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(n) = line?.parse() {
            let len = v.len();
            if len >= 25 {
                let mut good = false;
                'outer: for i in len - 25..len - 1 {
                    for j in i..len {
                        if v[i] != v[j] && v[i] + v[j] == n {
                            good = true;
                            break 'outer;
                        }
                    }
                }
                if !good {
                    println!("{}", n);
                    break;
                }
            }
            v.push(n);
        }
    }

    return Ok(());
}

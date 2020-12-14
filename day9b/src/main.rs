use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut v: Vec<i64> = Vec::new();

    let mut target = None;

    for line in stdin.lock().lines() {
        if let Ok(n) = line?.parse() {
            let len = v.len();
            if target.is_none() && len >= 25 {
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
                    target = Some(n);
                }
            }
            v.push(n);
        }
    }

    let target = target.unwrap();

    let (mut i, mut j) = (0, 0);
    let mut sum = 0;

    loop {
        if sum > target {
            sum -= v[i];
            i += 1;
        } else if sum < target {
            sum += v[j];
            j += 1;
        } else {
            let (mut min, mut max) = (v[i], v[i]);
            for k in i..j {
                if v[k] < min {
                    min = v[k]
                }
                if v[k] > max {
                    max = v[k]
                }
            }
            println!("{}", min + max);
            break;
        }
    }

    return Ok(());
}

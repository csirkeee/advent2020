use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let (mut or_mask, mut and_mask): (u64, u64) = (0, (1 << 36) - 1);

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                if let [target, value, ..] = line.split(" = ").collect::<Vec<&str>>()[..] {
                    if target == "mask" {
                        or_mask = 0;
                        and_mask = 0;
                        for ch in value.chars() {
                            and_mask *= 2;
                            or_mask *= 2;
                            match ch {
                                'X' => {
                                    and_mask += 1;
                                }
                                '1' => {
                                    and_mask += 1;
                                    or_mask += 1;
                                }
                                '0' => {}
                                _ => {}
                            }
                        }
                    } else {
                        let address: u64 = target
                            .trim_end_matches(']')
                            .split('[')
                            .nth(1)
                            .unwrap()
                            .parse()
                            .unwrap();
                        let value: u64 = value.parse().unwrap();
                        memory.insert(address, (value & and_mask) | or_mask);
                    }
                }
            }
        }
    }

    println!("{}", memory.values().sum::<u64>());
    return Ok(());
}

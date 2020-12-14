use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut float_bits: Vec<u64> = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                if let [target, value, ..] = line.split(" = ").collect::<Vec<&str>>()[..] {
                    if target == "mask" {
                        or_mask = 0;
                        float_bits.clear();
                        for ch in value.chars() {
                            or_mask *= 2;
                            for i in 0..float_bits.len() {
                                float_bits[i] *= 2;
                            }
                            match ch {
                                'X' => {
                                    float_bits.push(1);
                                }
                                '1' => {
                                    or_mask += 1;
                                }
                                '0' => {}
                                _ => {}
                            }
                        }
                    } else {
                        let mut address: u64 = target
                            .trim_end_matches(']')
                            .split('[')
                            .nth(1)
                            .unwrap()
                            .parse()
                            .unwrap();
                        address = address | or_mask;
                        let value: u64 = value.parse().unwrap();
                        for floating in 0..(1 << float_bits.len()) {
                            for bit_idx in 0..float_bits.len() {
                                if (floating & (1 << bit_idx)) > 0 {
                                    address = address ^ float_bits[bit_idx];
                                    break;
                                }
                            }
                            memory.insert(address, value);
                        }
                    }
                }
            }
        }
    }

    println!("{}", memory.values().sum::<u64>());
    return Ok(());
}

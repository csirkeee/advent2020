use std::error::Error;
use std::io;
use std::io::BufRead;

const NECESSARY_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut validated_fields: [bool; 7] = [false; 7];
    let mut valid_count = 0;

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() == 0 {
                // New passport next
                if !validated_fields.contains(&false) {
                    valid_count += 1;
                }
                validated_fields = [false; 7];
            } else {
                for field in line.split(' ') {
                    let field_name = field.split(':').next().unwrap();
                    for i in 0..NECESSARY_FIELDS.len() {
                        if field_name == NECESSARY_FIELDS[i] {
                            validated_fields[i] = true;
                        }
                    }
                }
            }
        }
    }

    if !validated_fields.contains(&false) {
        valid_count += 1;
    }

    println!("{}", valid_count);
    return Ok(());
}

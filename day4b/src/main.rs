use std::error::Error;
use std::io;
use std::io::BufRead;
use std::str;

const NECESSARY_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const CHECKERS: [fn(&str) -> bool; 7] = [
    |byr: &str| {
        byr.parse::<i32>()
            .map_or(false, |year| year >= 1920 && year <= 2002)
    },
    |iyr: &str| {
        iyr.parse::<i32>()
            .map_or(false, |year| year >= 2010 && year <= 2020)
    },
    |eyr: &str| {
        eyr.parse::<i32>()
            .map_or(false, |year| year >= 2020 && year <= 2030)
    },
    |hgt: &str| {
        if hgt.len() < 4 {
            return false;
        }
        let bytes = hgt.as_bytes();
        let unit = &bytes[bytes.len() - 2..bytes.len()];
        if unit == b"cm" {
            str::from_utf8(&bytes[0..bytes.len() - 2]).map_or(false, |size_str| {
                size_str
                    .parse::<i32>()
                    .map_or(false, |size| size >= 150 && size <= 193)
            })
        } else if unit == b"in" {
            str::from_utf8(&bytes[0..bytes.len() - 2]).map_or(false, |size_str| {
                size_str
                    .parse::<i32>()
                    .map_or(false, |size| size >= 59 && size <= 76)
            })
        } else {
            false
        }
    },
    |hcl: &str| {
        let hcl = hcl.as_bytes();
        hcl.len() == 7
            && hcl[0] == b'#'
            && hcl[1..7]
                .iter()
                .all(|&b| (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f'))
    },
    |ecl: &str| {
        ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth"
    },
    |pid: &str| {
        let pid = pid.as_bytes();
        pid.len() == 9 && pid.iter().all(|&b| b >= b'0' && b <= b'9')
    },
];

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
                    let mut field_parts = field.split(':');
                    let field_name = field_parts.next().unwrap();
                    let field_value = field_parts.next().unwrap();
                    for i in 0..NECESSARY_FIELDS.len() {
                        if field_name == NECESSARY_FIELDS[i] {
                            if CHECKERS[i](field_value) {
                                validated_fields[i] = true;
                            }
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

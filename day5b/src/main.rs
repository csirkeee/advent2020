use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut ids = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                let (mut row, mut col) = (0, 0);
                let (mut row_digit, mut col_digit) = (64, 4);

                for ch in line.chars() {
                    match ch {
                        'F' => {
                            row_digit /= 2;
                        }
                        'B' => {
                            row += row_digit;
                            row_digit /= 2;
                        }
                        'L' => {
                            col_digit /= 2;
                        }
                        'R' => {
                            col += col_digit;
                            col_digit /= 2;
                        }
                        _ => {}
                    }
                }

                let id = 8 * row + col;
                ids.push(id);
            }
        }
    }

    ids.sort();

    for i in 0..ids.len() - 1 {
        if ids[i + 1] > ids[i] + 1 {
            println!("{}", ids[i] + 1);
        }
    }
    return Ok(());
}

use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut group = [false; 26];
    let mut total_answers = 0;
    let mut first_in_group = true;

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() == 0 {
                // End of group
                total_answers += group.iter().filter(|&&a| a).count();
                group = [false; 26];
                first_in_group = true;
            } else {
                let mut guy = [false; 26];
                for ch in line.as_bytes() {
                    guy[(ch - b'a') as usize] = true;
                }
                if first_in_group {
                    group = guy;
                    first_in_group = false;
                } else {
                    for i in 0..26 {
                        group[i] = group[i] && guy[i];
                    }
                }
            }
        }
    }

    total_answers += group.iter().filter(|&&a| a).count();

    println!("{}", total_answers);
    return Ok(());
}

use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut group = [false; 26];
    let mut total_answers = 0;

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() == 0 {
                // End of group
                total_answers += group.iter().filter(|&&a| a).count();
                group = [false; 26];
            } else {
                for ch in line.as_bytes() {
                    group[(ch - b'a') as usize] = true;
                }
            }
        }
    }

    total_answers += group.iter().filter(|&&a| a).count();

    println!("{}", total_answers);
    return Ok(());
}

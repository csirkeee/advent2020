use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut good = 0;

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(&['-', ':', ' '][..]).collect();
            let min: i32 = parts[0].trim().parse()?;
            let max: i32 = parts[1].trim().parse()?;
            let letter = parts[2].trim().chars().next().unwrap();
            let pw = parts[4].trim();

            let mut count = 0;
            for ch in pw.chars() {
                if ch == letter {
                    count += 1;
                }
            }

            if count >= min && count <= max {
                good += 1;
            }
        }
    }

    println!("{}", good);
    return Ok(());
}

use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut good = 0;

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(&['-', ':', ' '][..]).collect();
            let first: i32 = parts[0].trim().parse()?;
            let second: i32 = parts[1].trim().parse()?;
            let letter = parts[2].trim().chars().next().unwrap();
            let pw = parts[4].trim();

            let mut count = 0;
            if pw.chars().nth((first - 1) as usize).unwrap() == letter {
                count += 1;
            }
            if pw.chars().nth((second - 1) as usize).unwrap() == letter {
                count += 1;
            }

            if count == 1 {
                good += 1;
            }
        }
    }

    println!("{}", good);
    return Ok(());
}

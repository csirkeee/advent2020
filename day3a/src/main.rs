use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut treemap: Vec<Vec<bool>> = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                let mut treeline = Vec::new();

                for ch in line.chars() {
                    treeline.push(ch == '#');
                }

                treemap.push(treeline);
            }
        }
    }

    let (mut x, mut y) = (0, 0);

    let mut count = 0;
    let width = treemap[0].len();
    while y < treemap.len() {
        if treemap[y][x] {
            count += 1;
        }
        x = (x + 3) % width;
        y += 1;
    }

    println!("{}", count);
    return Ok(());
}

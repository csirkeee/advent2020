use std::error::Error;
use std::io;
use std::io::BufRead;

fn count_trees(treemap: &Vec<Vec<bool>>, vx: usize, vy: usize) -> i64 {
    let (mut x, mut y) = (0, 0);

    let mut count = 0;
    let width = treemap[0].len();
    while y < treemap.len() {
        if treemap[y][x] {
            count += 1;
        }
        x = (x + vx) % width;
        y += vy;
    }
    count
}

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

    println!(
        "{}",
        count_trees(&treemap, 1, 1)
            * count_trees(&treemap, 3, 1)
            * count_trees(&treemap, 5, 1)
            * count_trees(&treemap, 7, 1)
            * count_trees(&treemap, 1, 2)
    );
    return Ok(());
}

use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let start: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut best = start * 2;
    let mut best_id = 0;

    for bus in lines.next().unwrap().unwrap().split(',') {
        if let Ok(id) = bus.parse() {
            let next_time = ((start / id) + 1) * id;
            if next_time < best {
                best = next_time;
                best_id = id;
            }
        }
    }

    println!(
        "{} * {} = {}",
        best_id,
        best - start,
        best_id * (best - start)
    );

    return Ok(());
}

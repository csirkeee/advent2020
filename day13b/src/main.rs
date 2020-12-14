use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();

    let mut lcm_so_far: i64 = 1;
    let mut time: i64 = 0;

    for (idx, bus) in lines.next().unwrap().unwrap().split(',').enumerate() {
        if let Ok(id) = bus.parse() {
            while ((time + (idx as i64)) % id) != 0 {
                time += lcm_so_far;
            }
            lcm_so_far = num_integer::lcm(lcm_so_far, id);
        }
    }

    println!("{}", time);

    return Ok(());
}

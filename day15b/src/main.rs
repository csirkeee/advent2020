use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut last_time: HashMap<i32, i32> = HashMap::new();
    let mut time = 0;
    let mut next_value = 0;

    let first_line = stdin.lock().lines().next().unwrap().unwrap();
    let first_line = first_line.trim();

    for number_string in first_line.split(',') {
        if let Ok(i) = number_string.parse() {
            time += 1;
            let current_value = i;
            next_value = match last_time.get(&current_value) {
                Some(last) => time - last,
                None => 0,
            };
            last_time.insert(current_value, time);
        }
    }

    while time < 29999999 {
        time += 1;
        let current_value = next_value;
        next_value = match last_time.get(&current_value) {
            Some(last) => time - last,
            None => 0,
        };
        last_time.insert(current_value, time);
    }

    println!("{}", next_value);
    return Ok(());
}

use std::error::Error;
use std::fmt::{Debug, Formatter, Write};
use std::io;
use std::io::BufRead;

#[derive(Clone, PartialEq, Eq)]
struct State {
    fields: Vec<Option<bool>>,
    width: usize,
    height: usize,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl State {
    fn read() -> Result<Self, Box<dyn Error>> {
        let mut fields = Vec::new();

        let mut width = 0;

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let line = line.trim().as_bytes();
            if line.len() > 0 {
                if width == 0 {
                    width = line.len() + 2;
                    // Add in first line
                    for _ in 0..width {
                        fields.push(None)
                    }
                }

                fields.push(None);
                for byte in line {
                    match *byte {
                        b'L' => fields.push(Some(false)),
                        _ => fields.push(None),
                    }
                }
                fields.push(None);

                if fields.len() % width != 0 {
                    panic!("Wrong line length!")
                }
            }
        }

        // Add in last line
        for _ in 0..width {
            fields.push(None)
        }

        Ok(State {
            width,
            height: fields.len() / width,
            fields,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.fields[y * self.width + x]
    }

    fn live(&self) -> Self {
        let mut fields = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                fields.push(match self.get(x, y) {
                    None => None,
                    Some(false) => Some(self.filled_neighbors(y, x) == 0),
                    Some(true) => Some(self.filled_neighbors(y, x) < 4),
                })
            }
        }

        State {
            fields,
            width: self.width,
            height: self.height,
        }
    }

    fn filled_neighbors(&self, y: usize, x: usize) -> i32 {
        let mut filled = 0;
        for &(dx, dy) in &NEIGHBORS {
            if Some(true) == self.get((x as i32 + dx) as usize, (y as i32 + dy) as usize) {
                filled += 1
            }
        }
        filled
    }

    fn count_filled(&self) -> usize {
        self.fields.iter().filter(|o| o.unwrap_or(false)).count()
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (i, field) in self.fields.iter().enumerate() {
            f.write_char(match field {
                None => '.',
                Some(false) => 'L',
                Some(true) => '#',
            })?;
            if (i + 1) % self.width == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::read()?;

    loop {
        // println!("{:?}", &state);

        let new_state = state.live();

        if new_state == state {
            println!("finished! {}", state.count_filled());
            break;
        }

        state = new_state;
    }

    return Ok(());
}

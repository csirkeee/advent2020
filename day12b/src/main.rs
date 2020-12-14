use std::error::Error;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Op {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

fn left((mut x, mut y): (i32, i32), amount: i32) -> (i32, i32) {
    for _ in 0..amount {
        let (nx, ny) = (-y, x);
        x = nx;
        y = ny;
    }
    (x, y)
}

fn right((mut x, mut y): (i32, i32), amount: i32) -> (i32, i32) {
    for _ in 0..amount {
        let (nx, ny) = (y, -x);
        x = nx;
        y = ny;
    }
    (x, y)
}

impl Op {
    fn parse(line: &str) -> Result<Self, Box<dyn Error>> {
        let mut it = line.chars();
        let op_ch = it.next().unwrap();
        let amount = it.as_str().parse()?;
        match op_ch {
            'N' => Ok(Op::North(amount)),
            'S' => Ok(Op::South(amount)),
            'E' => Ok(Op::East(amount)),
            'W' => Ok(Op::West(amount)),
            'L' => Ok(Op::Left(amount / 90)),
            'R' => Ok(Op::Right(amount / 90)),
            'F' => Ok(Op::Forward(amount)),
            _ => Err("Unknown op".into()),
        }
    }

    fn exec(&self, state: &State) -> State {
        match *self {
            Op::North(amount) => State {
                x: state.x,
                y: state.y,
                wx: state.wx,
                wy: state.wy + amount,
            },
            Op::South(amount) => State {
                x: state.x,
                y: state.y,
                wx: state.wx,
                wy: state.wy - amount,
            },
            Op::East(amount) => State {
                x: state.x,
                y: state.y,
                wx: state.wx + amount,
                wy: state.wy,
            },
            Op::West(amount) => State {
                x: state.x,
                y: state.y,
                wx: state.wx - amount,
                wy: state.wy,
            },
            Op::Left(amount) => {
                let (new_wx, new_wy) = left((state.wx, state.wy), amount);
                State {
                    x: state.x,
                    y: state.y,
                    wx: new_wx,
                    wy: new_wy,
                }
            }
            Op::Right(amount) => {
                let (new_wx, new_wy) = right((state.wx, state.wy), amount);
                State {
                    x: state.x,
                    y: state.y,
                    wx: new_wx,
                    wy: new_wy,
                }
            }
            Op::Forward(amount) => State {
                x: state.x + amount * state.wx,
                y: state.y + amount * state.wy,
                wx: state.wx,
                wy: state.wy,
            },
        }
    }
}

impl State {
    fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut state = State {
        x: 0,
        y: 0,
        wx: 10,
        wy: 1,
    };

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                let op = Op::parse(line)?;
                state = op.exec(&state);
            }
        }
    }

    println!("{:?} {}", state, state.dist());
    return Ok(());
}

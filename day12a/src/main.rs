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
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Dir {
    fn left(&self, amount: i32) -> Self {
        let mut curr = *self;
        for _ in 0..amount {
            curr = match curr {
                Dir::N => Dir::W,
                Dir::W => Dir::S,
                Dir::S => Dir::E,
                Dir::E => Dir::N,
            }
        }
        curr
    }

    fn right(&self, amount: i32) -> Self {
        let mut curr = *self;
        for _ in 0..amount {
            curr = match curr {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            }
        }
        curr
    }
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
                y: state.y + amount,
                dir: state.dir,
            },
            Op::South(amount) => State {
                x: state.x,
                y: state.y - amount,
                dir: state.dir,
            },
            Op::East(amount) => State {
                x: state.x + amount,
                y: state.y,
                dir: state.dir,
            },
            Op::West(amount) => State {
                x: state.x - amount,
                y: state.y,
                dir: state.dir,
            },
            Op::Left(amount) => State {
                x: state.x,
                y: state.y,
                dir: state.dir.left(amount),
            },
            Op::Right(amount) => State {
                x: state.x,
                y: state.y,
                dir: state.dir.right(amount),
            },
            Op::Forward(amount) => match state.dir {
                Dir::N => Op::North(amount).exec(state),
                Dir::S => Op::South(amount).exec(state),
                Dir::E => Op::East(amount).exec(state),
                Dir::W => Op::West(amount).exec(state),
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
        dir: Dir::E,
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

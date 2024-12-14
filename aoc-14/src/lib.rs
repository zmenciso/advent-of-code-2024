use derive_more::{Add, Sub};
use std::fmt;

#[derive(Debug, Add, Sub, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(input: &str) -> Coord {
        input.split_once(',')
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Robot {
    pub fn new(line: &str) -> Robot {
        let iter = line.split_whitespace();

        let p = iter.next().expect("Improperly formatted line");
        let pos = Coord::new(p.strip_prefix("p="));
    }
}

use derive_more::{Add, Sub};
use std::cmp::PartialEq;
use std::fmt;

pub const N: i32 = 103;
pub const M: i32 = 101;

#[derive(Debug, Add, Sub, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(input: &str) -> Coord {
        let (x, y) = input.split_once(',')
            .expect("Improperly formatted coord");

        let x = x.parse::<i32>()
            .expect("Could not parse x");

        let y = y.parse::<i32>()
            .expect("Could not parse x");

        Coord{x,y}
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    pub pos: Coord,
    pub vel: Coord,
}

impl Robot {
    pub fn new(line: &str) -> Robot {
        let mut iter = line.split_whitespace();

        let p = iter.next().expect("Improperly formatted line");
        let pos = Coord::new(p.strip_prefix("p=").expect("Could not find prefix"));

        let l = iter.next().expect("Improperly formatted line");
        let vel = Coord::new(l.strip_prefix("v=").expect("Could not find prefix"));

        Robot {pos, vel}
    }

    pub fn advance(&mut self) {
        let mut next = self.pos + self.vel;

        next.x = next.x.rem_euclid(M);
        next.y = next.y.rem_euclid(N);

        self.pos = next;
    }

    pub fn is_at(&self, loc: Coord) -> bool {
        self.pos == loc
    }
}

use std::fmt;
use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord{x,y}
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
pub struct Route {
    obs: Vec<Coord>,
    n: usize,
    m: usize,
    dir: Coord,
    pub loc: Coord,
}

impl Route {
    pub fn new() -> Route {
        Route {
            obs: Vec::new(),
            loc: Coord::new(0,0),
            dir: Coord::new(0,0),
            n: 0,
            m: 0
        }
    }

    fn decode_dir(c: char) -> Coord {
        match c {
            '>' => Coord::new(0, 1),
            '^' => Coord::new(-1, 0),
            '<' => Coord::new(0, -1),
            'v' => Coord::new(1, 0),
            _ => Coord::new(0, 0)
        }
    }

    fn find_obstacles(&mut self, line: &str) {
        // Update starting position
        for (index, c) in line.char_indices() {
            if (&['v', '<', '>', '^']).contains(&c) {
                self.dir = Self::decode_dir(c);
                self.loc = Coord::new(self.n as i32, index as i32);
            }
        }

        let v: Vec<i32> = line.char_indices()
            .filter(|&(_, c)| c == '#')
            .map(|(j, _)| j as i32)
            .collect();

        for j in v {
            let c = Coord::new(self.n as i32, j);
            self.obs.push(c);
        }
    }

    pub fn read_line(&mut self, line: &str) {
        if line.len() < 1 {
            return
        }

        Self::find_obstacles(self, line);

        self.n += 1;
        self.m = line.len();
    }

    /// Returns None if cannot move more
    pub fn step(&mut self) -> Option<Coord> {
        // Check for obstacle
        let next: Coord = self.loc + self.dir;
        if self.obs.contains(&next) {
            // Rotate 90 degrees
            self.dir = match self.dir {
                Coord{x:0, y:1} => Coord::new(1, 0),
                Coord{x:-1, y:0} => Coord::new(0, 1),
                Coord{x:0, y:-1} => Coord::new(-1, 0),
                Coord{x:1, y:0} => Coord::new(0, -1),
                _ => Coord::new(0, 0)
            };

            Some(self.loc)
        }
        else if (next.x <= self.n as i32 && next.x >= 0) && (next.y <= self.m as i32 && next.y >= 0) {
            // Take one step foward
            self.loc = next;
            Some(next)
        }
        else {
            // Cannot move more
            None
        }
    }
}

use core::str;
use std::fmt;
use std::ops::{Sub, Add};

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub i: i32,
    pub j: i32
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Coord {
        Coord {
            i: i as i32,
            j: j as i32
        }
    }

    pub fn diagonal(&self) -> Vec<Coord> {
        let mut v: Vec<Coord> = Vec::new();
        let i_var = [self.i - 1, self.i + 1];
        let j_var = [self.j - 1, self.j + 1];

        for &i in &i_var {
            for &j in &j_var {
                v.push(Coord{i,j});
            }
        }
        v
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    g: Vec<Vec<u8>>,
    pub n: usize,
    pub m: usize
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            g: Vec::new(),
            n: 0,
            m: 0
        }
    }

    pub fn insert_row(&mut self, row: &str) {
        self.g.push(row.as_bytes().to_owned());
        self.n += 1;
        if self.m == 0 {
            self.m = row.len();
        }
        else {
            assert_eq!(self.m, row.len());
        }
    }

    pub fn retrieve(&self, loc: Coord) -> Option<u8> {
        if (loc.i < self.n as i32 && loc.j < self.m as i32) 
        && loc.i >= 0 && loc.j >= 0 {
            return Some(self.g[loc.i as usize][loc.j as usize])
        }
        else { return None }
    }

    /// Returns a list of coordinates that match the search character
    pub fn search(&mut self, loc: Coord, c: u8) -> Vec<Coord> {
        let mut v: Vec<Coord> = loc.diagonal();
        v.retain(|&x| self.retrieve(x) == Some(c));
        v
    }

    pub fn search_seq(&mut self, loc: Coord, seq: &[u8], center: usize) -> bool {
        let mut count: usize = 0;
        // Find all matches for char left of center
        let v: Vec<Coord> = Self::search(self, loc, seq[center-1]);

        for coord in v {
            // Compute OPPOSITE direction
            let dir: Coord = loc - coord;
            let mut search: Coord = loc;

            // Search along that direction
            for i in center + 1 .. seq.len() {
                search = search + dir;
                if Self::retrieve(&self, search) != Some(seq[i]) { break }
                if i + 1 == seq.len() { count += 1 }
            }
        }

        count == 2
    }

    pub fn search_all(&mut self, seq: &[u8]) -> usize {
        let mut count: usize = 0;
        let center: usize = seq.len() / 2;

        for i in 0 .. self.n {
            for j in 0 .. self.m {
                let loc = Coord::new(i, j);
                if self.retrieve(loc) == Some(seq[center]) {
                    if Self::search_seq(self, loc, seq, center) { count += 1 };
                }
            }
        }
        
        count
    }
}

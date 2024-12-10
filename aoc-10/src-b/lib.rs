use std::fmt;
use derive_more::{Add, Sub};

#[derive (Debug, Add, Sub, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Coord {
    x: i32,
    y: i32
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive (Debug)]
pub struct Map {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message = String::from("");

        for row in &self.map {
            message = format!("{}\n", message);

            for num in row {
                message = format!("{}{}", message, num);
            }
        }

        write!(f, "{}", message)
    }
}

impl Map {
    pub fn new() -> Map {
        Map {
            map: Vec::new(),
            m: 0,
            n: 0
        }
    }

    pub fn inspect(&self, loc: &Coord) -> Option<u8> {
        // Check bounds
        if loc.x >= self.n as i32 || loc.y >= self.m as i32 || loc.x < 0 || loc.y < 0 {
            return None
        }
        else {
            return Some(self.map[loc.x as usize][loc.y as usize])
        }
    }

    pub fn read_line(&mut self, line: &str) {
        let v: Vec<u8> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as u8)
            .collect();

        if self.m == 0 {
            self.m = v.len();
        }
        else {
            assert_eq!(self.m, v.len());
        }

        self.map.push(v);
        self.n += 1;
    }

    pub fn trailheads(&self) -> Vec<Coord> {
        let mut v: Vec<Coord> = Vec::new();

        for i in 0 .. self.n {
            for j in 0 .. self.m {
                if self.map[i][j] == 0 { 
                    let coord = Coord{x: i as i32, y: j as i32};
                    v.push(coord);
                }
            }
        }

        v
    }

    /// If at loc, returns all coordinates that satify the slope rules
    pub fn slope(&self, loc: &Coord) -> Vec<Coord> {
        if let Some(height) = Self::inspect(self, loc) {
            let check = [
                Coord{x: -1, y: 0} + *loc,
                Coord{x: 1, y: 0} + *loc,
                Coord{x: 0, y: -1} + *loc,
                Coord{x: 0, y: 1} + *loc
            ];

            check.iter()
                .filter_map(|&x| {
                    match Self::inspect(self, &x) {
                        Some(val) if (val as i32 - height as i32) == 1 => Some(x),
                        _ => None,
                    }
                })
                .collect()
        }
        else {
            return Vec::new()
        }
    }
}

use std::fmt;
use derive_more::{Add, Sub};

#[derive (Debug, Add, Sub, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive (Debug)]
pub struct Map {
    map: Vec<Vec<char>>,
    pub m: usize,
    pub n: usize
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

    pub fn inspect(&self, loc: &Coord) -> Option<char> {
        // Check bounds
        if loc.x >= self.n as i32 || loc.y >= self.m as i32 || loc.x < 0 || loc.y < 0 {
            return None
        }
        else {
            return Some(self.map[loc.x as usize][loc.y as usize])
        }
    }

    pub fn read_line(&mut self, line: &str) {
        let v: Vec<char> = line.chars().collect();

        if self.m == 0 {
            self.m = v.len();
        }
        else {
            assert_eq!(self.m, v.len());
        }

        self.map.push(v);
        self.n += 1;
    }

    pub fn adjacent(&self, loc: &Coord) -> Vec<Coord> {
        vec![
            *loc + Coord{x: 1, y: 0},
            *loc - Coord{x: 1, y: 0},
            *loc + Coord{x: 0, y: 1},
            *loc - Coord{x: 0, y: 1},
        ]

        /*
        check.iter()
            .filter(|&c| Self::inspect(self, c).is_some())
            .map(|x| *x)
            .collect()
        */
    }
}

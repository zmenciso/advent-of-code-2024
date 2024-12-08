use std::collections::{HashMap, HashSet};
use std::fmt;
use derive_more::{Add, Sub};

#[derive (Debug, Eq, PartialEq, Add, Sub, Clone, Copy, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive (Debug)]
pub struct Map {
    n: usize,
    m: usize,
    dirty: HashSet<char>,
    ant: HashMap<char, Vec<Coord>>,
    nod: HashSet<Coord>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            n: 0,
            m: 0,
            dirty: HashSet::new(),
            ant: HashMap::new(),
            nod: HashSet::new(),
        }
    }

    pub fn read_line(&mut self, line: &str) {
        if self.m == 0 { self.m = line.len() } 
        else { assert_eq!(self.m, line.len()) }

        for (i, c) in line.char_indices().filter(|&(_, c) | c != '.') {
            // Tag c as dirty
            self.dirty.insert(c);

            let coord = Coord{x: i as i32, y: self.n as i32};

            if let Some(v) = self.ant.get_mut(&c) {
                v.push(coord);
            }
            else {
                self.ant.insert(c, vec![coord]);
            }
        }

        self.n += 1;
    }

    fn update_nodes(&mut self, c: &char) {
        if !self.ant.contains_key(c) { return }

        let ant = self.ant.get(c).unwrap();

        // Generate all possible 2-antenna combinations
        for i in 0 .. ant.len() {
            for j in (i + 1) .. ant.len() {
                let (a, b) = (&ant[i], &ant[j]);

                let nodes = antinodes(a, b).into_iter()
                    .filter(|x| x.x < self.n as i32 && x.y < self.m as i32 && x.x >= 0 && x.y >= 0);

                for node in nodes { self.nod.insert(node); }
            }
        }

        self.dirty.remove(c);
    }

    pub fn antinodes(&mut self) -> &HashSet<Coord> {
        // TODO: Remove the clone
        let dirty: Vec<_> = self.dirty.iter().cloned().collect();

        for c in dirty {
            Self::update_nodes(self, &c);
        }

        &self.nod
    }
}

fn antinodes(a: &Coord, b: &Coord) -> Vec<Coord> {
    let del = *a - *b;
    vec![*a+del, *b-del]
}

use std::io;
use aoc_06::{Route, Coord};


fn main() {
    let mut r = Route::new();
    let mut visited: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not parse line");
        r.read_line(&line);
    }

    visited.push(r.loc);
    while let Some(loc) = r.step() {
        if !visited.contains(&loc) { visited.push(loc) }
    }

    println!("{:?}", visited.len());
}

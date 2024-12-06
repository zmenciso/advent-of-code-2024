use std::io;
use std::collections::HashSet;
use aoc_06::{Route, Coord};

const MIN_CYCLE: usize = 4;

/// Returns true if cycle detected
fn detect_cycle(v: &Vec<Coord>) -> bool {
    if v.len() < MIN_CYCLE * 2 { return false }

    let end: usize = v.len().checked_sub(1)
        .expect("Underflow!");

    for n in MIN_CYCLE .. v.len()/2 {
        let i = end - n;
        let tail = &v[i+1 .. end];
        let head = &v[i-n+1 .. i];

        if tail == head { return true }
    }
    false
}

/// Returns true if obstacle successfully blocks
fn try_obstacle(r: &mut Route, obs: Coord) -> bool {
    r.reset();
    r.obs.push(obs);

    let mut v: Vec<Coord> = Vec::new();

    while let Some(loc) = r.step() {
        v.push(loc);
        if detect_cycle(&v) { 
            r.obs.pop();
            return true
        }
    }

    r.obs.pop();
    false
}

fn main() {
    let mut r = Route::new();
    let mut visited: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not parse line");
        r.read_line(&line);
    }

    // Run the path to get valid obstruction locations
    while let Some(loc) = r.step() {
        if !visited.contains(&loc) { visited.push(loc) }
    }

    // Brute force the solution
    let mut count = 0;
    for loc in visited {
        if try_obstacle(&mut r, loc) { count += 1 }
    }

    println!("{:?}", count);
}

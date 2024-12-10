use std::io;
use std::collections::HashSet;
use aoc_10::{Map, Coord};

/// Returns the trailhead score
fn score_trail(map: &Map, start: &Coord, peaks: &mut HashSet<Coord>) {
    let search = map.slope(start);
    for loc in search {
        let x = map.inspect(&loc);
        if x == Some(9) {
            peaks.insert(loc);
            continue
        }
        else if x.is_some() {
            score_trail(map, &loc, peaks);
        }
    }
}

fn main() {
    let mut map = Map::new();

	for line in io::stdin().lines() {
		let line = line.expect("Could not read line");
		if line.len() > 0 { map.read_line(&line) }
	}

	let trailheads = map.trailheads();

	let sum: usize = trailheads.iter()
	    .map(|start| {
            let mut peaks: HashSet<Coord> = HashSet::new();
            score_trail(&map, &start, &mut peaks);
            peaks.len()
        })
	    .sum();

	println!("{}", sum);
}

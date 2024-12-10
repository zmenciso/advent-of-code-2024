use std::io;
use std::collections::HashSet;
use aoc_10::{Map, Coord};

/// Returns the trailhead score
fn dfs(map: &Map, start: &Coord, path: &mut Vec<Coord>, paths: &mut Vec<Vec<Coord>>, visited: &mut HashSet<Coord>) {
    path.push(*start);
    if map.inspect(&start) == Some(9) {
        paths.push(path.clone());
        return
    }

    let search = map.slope(start);
    for loc in search {
        if !visited.contains(&loc) { 
            visited.insert(loc);
            dfs(map, &loc, path, paths, visited);
            visited.remove(&loc);
        }
    }

    path.pop();
}

fn main() {
    let mut map = Map::new();

	for line in io::stdin().lines() {
		let line = line.expect("Could not read line");
		if line.len() > 0 { map.read_line(&line) }
	}

	let trailheads = map.trailheads();
	let mut sum = 0;

	for head in trailheads {
        let mut path = Vec::new();
        let mut paths = Vec::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        visited.insert(head);
        dfs(&map, &head, &mut path, &mut paths, &mut visited);
        sum += paths.len();
    }

	println!("{}", sum);
}

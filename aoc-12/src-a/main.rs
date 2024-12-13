use std::io;
use std::collections::HashSet;
use aoc_12::{Coord, Map};

type Region = HashSet<Coord>;

fn find_region(map: &Map, loc: &Coord, region: &mut Region, seen: &mut Region, perimeter: &mut usize) -> Option<char> {
    let plant = map.inspect(loc);

    if plant.is_some() && !seen.contains(loc) {
        seen.insert(*loc);
        region.insert(*loc);

        for x in map.adjacent(loc) {
            if map.inspect(&x) == plant { 
                find_region(map, &x, region, seen, perimeter);
            }
            else {
                *perimeter += 1;
            }
        }
    }

    plant
}

fn main() {
    let mut map = Map::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        map.read_line(&line);
    }

    let mut seen: Region = HashSet::new();
    let mut regions: Vec<(char, usize, Region)> = Vec::new();
    let mut region: Region = HashSet::new();

    for i in 0 .. map.m {
        for j in 0 .. map.n {
            let loc = Coord{x: i as i32, y: j as i32};
            if seen.contains(&loc) { continue }

            let mut perimeter: usize = 0;
            let c: char = find_region(&map, &loc, &mut region, &mut seen, &mut perimeter)
                .expect("No plant found at location");

            regions.push((c, perimeter, region));
            region = HashSet::new();
        }
    }

    let sum: usize = regions.iter()
        .map(|(_, p, r)| p * r.len())
        .sum();

    println!("{}", sum);
}

use std::io;
use std::cmp::max;
use std::collections::HashSet;
use aoc_12::{Coord, Map};

type Region = HashSet<Coord>;

fn find_region(map: &Map, loc: &Coord, region: &mut Region, seen: &mut Region, perimeter: &mut Region) -> Option<char> {
    let plant = map.inspect(loc);

    if plant.is_some() && !seen.contains(loc) {
        seen.insert(*loc);
        region.insert(*loc);

        for x in loc.adjacent() {
            if map.inspect(&x) == plant { 
                find_region(map, &x, region, seen, perimeter);
            }
            else {
                perimeter.insert(x);
            }
        }
    }

    plant
}

fn neighbors(loc: &Coord, adj: &mut Vec<Coord>, region: &Region) {
    if adj.contains(loc) { return }

    adj.push(*loc);

    let n = loc.adjacent().iter()
        .filter(|&l| region.contains(l))
        .map(|x| *x)
        .collect::<Vec<Coord>>();

    for l in n {
        neighbors(&l, adj, region);
    }
}

fn cost(region: &Region, perimeter: &Region) -> usize {
    let mut sides: usize = 0;
    let check: Vec<Coord> = perimeter.iter()
        .map(|x| *x)
        .collect::<Vec<Coord>>();

    let mut seen: Region = HashSet::new();

    for loc in check {
        if seen.contains(&loc) { continue }

        // Find all coords in a line
        let mut line: Vec<Coord> = Vec::new();
        neighbors(&loc, &mut line, perimeter);

        let mut touching: usize = 0;

        for l in &line {
            seen.insert(*l);
            touching = max(touching, l.adjacent()
                .iter()
                .filter(|&x| region.contains(x))
                .collect::<Vec<&Coord>>()
                .len());
        }

        sides += touching;
    }

    return sides * region.len()
}

fn main() {
    let mut map = Map::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        map.read_line(&line);
    }

    let mut seen: Region = HashSet::new();
    let mut regions: Vec<(char, Region, Region)> = Vec::new();
    let mut region: Region = HashSet::new();

    for i in 0 .. map.m {
        for j in 0 .. map.n {
            let loc = Coord{x: i as i32, y: j as i32};
            if seen.contains(&loc) { continue }

            let mut perimeter = HashSet::new();
            let c: char = find_region(&map, &loc, &mut region, &mut seen, &mut perimeter)
                .expect("No plant found at location");

            regions.push((c, perimeter, region));
            region = HashSet::new();
        }
    }

    // for (c, p ,r) in regions {
    //     println!("{} {}", c, cost(&r, &p));
    // }

    let sum: usize = regions.iter()
        .map(|(_, p, r)| cost(&r, &p))
        .sum();

    println!("{}", sum);
}

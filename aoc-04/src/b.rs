use std::io;
use aoc_04::Grid;

fn main() {
    let mut g = Grid::new();

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        g.insert_row(&line);
    }

    println!("{}", g.search_all("MAS".as_bytes()));
}

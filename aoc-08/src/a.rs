use std::io;
use aoc_08::Map;

fn main() {
    let mut m = Map::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not parse line");
        m.read_line(&line);
    }
    
    println!("{:?}", m.antinodes().len());
}

use std::io;
use aoc_11::{Stone};

const BLINKS: usize = 25;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Could not read line");

    let mut stones: Vec<Stone> = line.split_whitespace()
        .map(|s| Stone::new(s))
        .collect();

    for _ in 0 .. BLINKS {
        let mut v: Vec<Stone> = Vec::new();
        for stone in stones {
            v.extend(stone.blink());
        }

        stones = v;
    }

    println!("{}", stones.len());
}

use std::io;
use aoc_11::Stone;
use std::collections::HashMap;

const BLINKS: usize = 75;

fn update_stones(hm: &mut HashMap<Stone, usize>, stone: Stone, count: usize) {
    if let Some(x) = hm.get_mut(&stone) {
        *x += count;
    }
    else {
        hm.insert(stone, count);
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Could not read line");

    let start = line.split_whitespace()
        .map(|s| Stone::new(s));

    let mut stones: HashMap<Stone, usize> = HashMap::new();
    for s in start {
        update_stones(&mut stones, s, 1);
    }

    let mut temp: HashMap<Stone, usize>;

    for _ in 0 .. BLINKS {
        temp = HashMap::new();

        for (stone, count) in stones.iter() {
            let x = stone.blink();
            for val in x {
                update_stones(&mut temp, val, *count);
            }
        }

        stones = temp;
    }

    let sum: usize = stones.values().sum();
    println!("{}", sum); 
}

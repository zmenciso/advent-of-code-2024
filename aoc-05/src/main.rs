use std::io;
use aoc_05::{Rules, Update};

fn main() {
    let mut rules = Rules::new();
    let mut middle: Vec<usize> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");

        if line.contains('|') {
            rules.insert(&line);
        } 
        else if line.contains(',') {
            let update = Update::new(&line);
            if update.check_valid(&rules) {
                middle.push(update.mid)
            }
        }
    }

    let sum: usize = middle.iter().sum();
    println!("{:?}", sum);
}

use std::io;
use aoc_13::Machine;

const A: i64 = 3;
const B: i64 = 1;

fn main() {
    let mut tokens = 0;
    let mut m = Machine::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");

        m.read_line(&line);

        if m.complete() {
            if let Some((a,b)) = m.solve() {
                tokens += a * A + b * B;
            }

            m = Machine::new();
        }
    }

    println!("{}", tokens);
}

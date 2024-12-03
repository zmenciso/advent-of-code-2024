use regex::Regex;
use std::io;
use aoc_03::Mult;

fn extract_inst(seq: &str, v: &mut Vec<Mult>) {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)")
        .expect("Could not create regex");

    for (_, [t]) in re.captures_iter(seq).map(|c| c.extract()) {
        let Some((a, b)) = t.split_once(',') else { return };
        let m = Mult::new(
            a.parse::<i32>().expect("Could not parse value"),
            b.parse::<i32>().expect("Could not parse value"),
        );

        v.push(m);
    }
}

fn main() {
    let mut v: Vec<Mult> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        extract_inst(&line, &mut v);
    }

    let sum: i32 = v.iter()
        .map(|m| m.product())
        .sum();

    println!("{}", sum);
}

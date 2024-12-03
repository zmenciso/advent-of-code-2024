use regex::Regex;
use std::io;
use aoc_03::Mult;

fn extract_inst(seq: &str, v: &mut Vec<Mult>, en: &mut bool) {

    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)|(do)\(\)|(don't)\(\)")
        .expect("Could not create regex");

    for (_, [t]) in re.captures_iter(seq).map(|c| c.extract()) {
        match t {
            "do" => { *en = true }
            "don't" => { *en = false }
            _ => {
                if *en {
                    let Some((a, b)) = t.split_once(',') else { return };
                    let m = Mult::new(a, b);

                    v.push(m);
                }
            }
        }
    }
}

fn main() {
    let mut v: Vec<Mult> = Vec::new();
    let mut en: bool = true;

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        extract_inst(&line, &mut v, &mut en);
    }

    let sum: i32 = v.iter()
        .map(|m| m.product())
        .sum();

    println!("{}", sum);
}

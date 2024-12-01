use std::io;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // Ingest input
    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();

        let n1 = iter.next()
            .expect("Missing number")
            .parse::<i32>()
            .expect("Could not parse number");

        let n2 = iter.next()
            .expect("Missing number")
            .parse::<i32>()
            .expect("Could not parse number");

        v1.push(n1);
        v2.push(n2);
    }

    // Sort vectors
    v1.sort();
    v2.sort();

    // Find the similarity scores
    let mut sim: Vec<usize> = Vec::new();
    for num in v1 {
        sim.push(num as usize * v2.iter().filter(|&x| *x == num).count())
    }

    let sum: usize = sim.iter().sum();

    println!("{}", sum);
}

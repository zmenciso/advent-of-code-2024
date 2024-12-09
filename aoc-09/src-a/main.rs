use std::io;
use aoc_09::Disk;

fn main() {
    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let mut disk = Disk::new(&line);

        disk.defrag();

        println!("{}", disk.checksum());
    }
}

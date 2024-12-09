#!/usr/bin/fish

set template "use std::io;

fn main() {
    for line in io::stdin().lines() {
        let line = line.expect(\"Could not read line\");
    }
}"

set dir "aoc-$argv[1]"
mkdir $dir
cd $dir

cargo init
echo $template > src/main.rs

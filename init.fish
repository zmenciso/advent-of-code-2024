#!/usr/bin/fish

set dir "aoc-$argv[1]"
mkdir $dir
cd $dir
cargo init

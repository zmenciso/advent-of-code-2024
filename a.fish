#!/usr/bin/env fish

set day (string split '-' (basename (pwd)))[2]

cp -r src/ src-a
git add src/* src-a/* Cargo.toml
git commit -m "Day $day: Part A"

#!/usr/bin/env fish

set day (string split '-' (basename (pwd)))[2]

cp -r src/ src-b
git add src/* src-b/* Cargo.toml
git commit -m "Day $day: Complete"

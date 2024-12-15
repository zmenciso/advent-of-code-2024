use std::io;
use std::{thread, time};
use aoc_14::{Coord, Robot};
use aoc_14::{N, M};

pub const STEPS: usize = 100;

fn safety_factor(robots: &Vec<Robot>) -> usize {
    let mut quadrants = [0, 0, 0, 0];

    for robot in robots {
        if robot.pos.x > M / 2 && robot.pos.y < N / 2 {
            quadrants[0] += 1;
        }
        else if robot.pos.x < M / 2 && robot.pos.y < N / 2 {
            quadrants[1] += 1;
        }
        else if robot.pos.x < M / 2 && robot.pos.y > N / 2 {
            quadrants[2] += 1;
        }
        else if robot.pos.x > M / 2 && robot.pos.y > N / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants.iter()
        .product()
}

fn print_map(robots: &Vec<Robot>) {
    for j in 0 .. N {
        for i in 0 .. M {
            let loc = Coord{x: i, y: j};
            let count = robots.iter()
                .filter(|&x| x.is_at(loc))
                .count();

            if count > 0 { print!("{}", count); }
            else { print!("."); }
        }
        println!();
    }
}

fn main() {
    let mut robots: Vec<Robot> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        robots.push(Robot::new(&line));
    }

    let mut i = 0;

    while true {
        for robot in &mut robots {
            println!("{} seconds", i);
            print_map(&robots);
            robot.advance();
            thread::sleep(time::Duration::from_secs(1));
        }
    }

}

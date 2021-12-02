use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

pub fn part1() -> i32 {
    let f = File::open("input/day2.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let cmds = f
        .lines()
        .collect::<std::io::Result<Vec<String>>>()
        .expect("Error parsing input");

    let mut pos = 0;
    let mut depth = 0;

    for cmd in cmds {
        let arg = cmd.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        let cmd = cmd.split(' ').nth(0).unwrap();

        if cmd == "forward" {
            pos += arg;
        }

        if cmd == "down" {
            depth += arg;
        } else if cmd == "up" {
            depth -= arg;
        }
    }
    pos * depth
}

pub fn part2() -> i32 {
    let f = File::open("input/day2.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let cmds = f
        .lines()
        .collect::<std::io::Result<Vec<String>>>()
        .expect("Error parsing input");

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in cmds {
        let arg = cmd.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        let cmd = cmd.split(' ').nth(0).unwrap();

        if cmd == "forward" {
            pos += arg;
            depth += aim * arg;
        }

        if cmd == "down" {
            aim += arg;
        } else if cmd == "up" {
            aim -= arg;
        }
    }
    pos * depth
}

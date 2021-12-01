use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

pub fn part1() -> i32 {
    let f = File::open("input/day1.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut increases = 0;
    let mut previous = None;

    for depth in f.lines() {
        let depth: i32 = depth.unwrap().trim().parse().expect("Unable to read line");

        if previous.is_none() {
            previous = Some(depth);
        }

        if depth > previous.unwrap() {
            increases += 1;
        }

        previous = Some(depth);
    }
    increases
}

pub fn part2() -> i32 {
    let f = File::open("input/day1.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let depths: Vec<i32> = f
        .lines()
        .map(|depth| depth.unwrap().parse::<i32>().unwrap())
        .collect();

    3
}

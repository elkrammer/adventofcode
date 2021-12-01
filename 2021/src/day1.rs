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
        .filter(|depth| depth.is_ok())
        .map(|depth| depth.unwrap())
        .map(|depth| depth.parse::<i32>())
        .filter(|depth| depth.is_ok())
        .map(|depth| depth.unwrap())
        .collect();

    let mut increases = 0;
    let mut prev = i32::MAX;

    for depth in depths.windows(3) {
        let curr: i32 = depth.iter().sum();
        if curr > prev {
            increases += 1
        }
        prev = curr;
    }
    increases
}

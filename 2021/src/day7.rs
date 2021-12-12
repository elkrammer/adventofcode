use std::fs;

pub fn day7() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

fn part1() -> i32 {
    let input = fs::read_to_string("input/day7.txt").unwrap();
    let crabs: Vec<i32> = input
        .trim()
        .split(',')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse().unwrap())
        .collect();

    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let mut best = i32::MAX;

    for point in min..=max {
        let mut cost = 0;
        for crab in &crabs {
            cost += (crab - point).abs();
        }
        best = best.min(cost);
    }

    best
}

fn part2() -> i32 {
    let input = fs::read_to_string("input/day7.txt").unwrap();
    let crabs: Vec<i32> = input
        .trim()
        .split(',')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse().unwrap())
        .collect();

    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let mut best = i32::MAX;

    for point in min..=max {
        let mut cost = 0;
        for crab in &crabs {
            let delta = (crab - point).abs();
            cost += delta * (delta + 1) / 2;
        }

        best = best.min(cost);
    }

    best
}

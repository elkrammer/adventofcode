use std::fs;

pub fn day1() -> String {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

pub fn part1(input: &str) -> i32 {
    let result = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();
    result
}

pub fn part2(input: &str) -> i32 {
    let mut result = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    result.sort_by(|a, b| b.cmp(a));
    let sum = result.iter().take(3).sum();
    sum
}

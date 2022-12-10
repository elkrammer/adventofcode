use std::collections::HashSet;
use std::fs;

pub fn day6() -> String {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

pub fn part1(input: &str) -> usize {
    let mut result: usize = 4;
    let signal: Vec<_> = input.chars().collect();
    for (_, char) in signal.windows(4).enumerate() {
        let unique: HashSet<_> = char.iter().collect();
        if unique.len() == 4 {
            break;
        }
        result += 1;
    }
    result
}

pub fn part2(input: &str) -> usize {
    let mut result: usize = 14;
    let signal: Vec<_> = input.chars().collect();
    for (_, char) in signal.windows(14).enumerate() {
        let unique: HashSet<_> = char.iter().collect();
        if unique.len() == 14 {
            break;
        }
        result += 1;
    }
    result
}

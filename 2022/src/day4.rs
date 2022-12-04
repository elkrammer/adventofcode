use std::fs;
use std::{str::FromStr, string::ParseError};

pub fn day4() -> String {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.start > other.end {
            return false;
        }

        if self.end < other.start {
            return false;
        }

        true
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s.split('-').map(|p| p.parse().unwrap()).collect();
        let [start, end] = &pair[..] else { unreachable!() };

        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}

pub fn part1(input: &str) -> usize {
    // let input = "2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8";

    let result: usize = input
        .lines()
        .map(|line| {
            let pair = line.split(',').map(|s| Range::from_str(s).unwrap());
            let [a, b] = &pair.collect::<Vec<_>>()[..] else { todo!() } ;
            (a.contains(b) || b.contains(a)) as usize
        })
        .sum();

    result
}

pub fn part2(input: &str) -> usize {
    let result: usize = input
        .lines()
        .map(|line| {
            let pair = line.split(',').map(|s| Range::from_str(s).unwrap());
            let [a, b] = &pair.collect::<Vec<_>>()[..] else { todo!() } ;
            (a.overlaps(b) || b.overlaps(a)) as usize
        })
        .sum();

    result
}

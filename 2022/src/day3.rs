use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn day3() -> String {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

pub fn part1(input: &str) -> usize {
    let alphabet = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let comp_1 = &line[0..sack_length];
            let comp_2 = &line[sack_length..(sack_length * 2)];

            let common_char = comp_1.chars().find(|c: &char| comp_2.contains(*c)).unwrap();

            alphabet.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result
}

pub fn part2(input: &str) -> usize {
    let alphabet = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let sacks: Vec<&str> = input.lines().collect();
    let result: usize = sacks
        .chunks(3)
        .map(|chunk| {
            let g1: HashSet<char> = chunk[0].chars().collect();
            let g2: HashSet<char> = chunk[1].chars().collect();
            let g3: HashSet<char> = chunk[2].chars().collect();

            let common = g1
                .intersection(&g2)
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&g3)
                .copied()
                .next()
                .expect("common in group");

            alphabet.get(&common).unwrap()
        })
        .sum::<usize>();
    result
}

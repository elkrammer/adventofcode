use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

fn parse_stacks(stack_input: &str) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();
    for line in stack_input.lines() {
        for (idx, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
                let idx = (idx - 1) / 4;
                if idx >= stacks.len() {
                    stacks.resize(idx + 1, VecDeque::new());
                }
                stacks[idx].push_back(char);
            }
        }
    }
    stacks
}

pub fn day5() -> String {
    let input = fs::read_to_string("input/day5.txt").unwrap();

    // let input = "    [D]
    // [N] [C]
    // [Z] [M] [P]
    // 1   2   3

    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2";

    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

pub fn part1(input: &str) -> String {
    let (stacks, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<VecDeque<char>> = parse_stacks(stacks);

    for line in procedure.lines() {
        let instruction: Vec<_> = line
            .split_ascii_whitespace()
            .flat_map(usize::from_str)
            .map(|u| u - 1)
            .collect();

        let [count, src_stack, dst_stack] = &instruction[..] else { unreachable!() };
        for _ in 0..*count + 1 {
            let a = stacks[*src_stack].pop_front().unwrap();
            stacks[*dst_stack].push_front(a);
        }
    }

    let result: String = stacks.iter().map(|s| s[0]).collect();
    result
}

pub fn part2(input: &str) -> String {
    let (stacks, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<VecDeque<char>> = parse_stacks(stacks);

    for line in procedure.lines() {
        let instruction: Vec<_> = line
            .split_ascii_whitespace()
            .flat_map(usize::from_str)
            .map(|u| u - 1)
            .collect();

        let [count, src_stack, dst_stack] = &instruction[..] else { unreachable!() };

        let mut tmp_vec = Vec::new();
        for _ in 0..*count + 1 {
            let a = stacks[*src_stack].pop_front().unwrap();
            tmp_vec.push(a);
        }

        tmp_vec.reverse();
        for tmp in tmp_vec {
            stacks[*dst_stack].push_front(tmp);
        }
    }

    let result: String = stacks.iter().map(|s| s[0]).collect();
    result
}
